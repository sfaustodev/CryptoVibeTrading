use leptos::*;
use leptos_meta::Style;
use web_sys::{CanvasRenderingContext2d, MouseEvent, WheelEvent, KeyboardEvent, HtmlCanvasElement, Element, DomRect};
use wasm_bindgen::{JsCast, JsValue};
use crate::types::{Tool, Color, Stroke, StrokePoint};

#[component]
pub fn Whiteboard(
    /// Width of the canvas in pixels
    #[prop(default = 1920)]
    width: u32,
    /// Height of the canvas in pixels
    #[prop(default = 1080)]
    height: u32,
) -> impl IntoView {
    let canvas_ref: NodeRef<leptos::html::Canvas> = create_node_ref();
    let (current_tool, set_current_tool) = create_signal(Tool::Pen);
    let (current_color, set_current_color) = create_signal(Color::Green);
    let (thickness, set_thickness) = create_signal(2.0);
    let (is_drawing, set_is_drawing) = create_signal(false);

    // Undo/Redo stacks
    let (strokes, set_strokes) = create_signal(Vec::<Stroke>::new());
    let (undo_stack, set_undo_stack) = create_signal(Vec::<Stroke>::new());

    // Pan/Zoom state
    let (offset_x, set_offset_x) = create_signal(0.0);
    let (offset_y, set_offset_y) = create_signal(0.0);
    let (zoom, set_zoom) = create_signal(1.0);

    // Current stroke being drawn
    let (current_stroke, set_current_stroke) = create_signal(Vec::<StrokePoint>::new());

    // Context menu state for "Summon Dragrok"
    let (show_context_menu, set_show_context_menu) = create_signal(false);
    let (context_menu_x, set_context_menu_x) = create_signal(0.0);
    let (context_menu_y, set_context_menu_y) = create_signal(0.0);
    let (dragrok_summoned, set_dragrok_summoned) = create_signal(false);

    // Fire particle system
    let (particles, set_particles) = create_signal(Vec::<FireParticle>::new());

    #[derive(Clone, Debug)]
    struct FireParticle {
        x: f64,
        y: f64,
        vx: f64,
        vy: f64,
        life: f64,
        max_life: f64,
        size: f64,
        color: String,
    }

    // =====================
    // DRAWING ENGINE
    // =====================

    let get_context = move || -> Option<CanvasRenderingContext2d> {
        canvas_ref.get().and_then(|canvas| {
            // Use leptos_html_element_to_web_sys pattern
            use leptos::leptos_dom::HtmlElement;
            let web_canvas = canvas.into_any();
            web_canvas.dyn_ref::<HtmlCanvasElement>().and_then(|canvas| {
                canvas.get_context("2d").ok().flatten()
                    .and_then(|ctx| ctx.dyn_into::<CanvasRenderingContext2d>().ok())
            })
        })
    };

    let draw_stroke = |ctx: &CanvasRenderingContext2d, stroke: &Stroke| {
        if stroke.points.is_empty() {
            return;
        }

        ctx.begin_path();
        ctx.set_line_width(stroke.thickness);
        ctx.set_line_cap("round");
        ctx.set_line_join("round");

        let color_str = format!("{}", stroke.color);
        ctx.set_stroke_style(&JsValue::from_str(&color_str));

        match stroke.tool {
            Tool::Pen | Tool::Eraser => {
                if stroke.tool == Tool::Eraser {
                    ctx.set_global_composite_operation("destination-out");
                    ctx.set_line_width(stroke.thickness * 5.0);
                } else {
                    ctx.set_global_composite_operation("source-over");
                }

                let mut points = stroke.points.iter().peekable();
                if let Some(first) = points.next() {
                    ctx.move_to(first.x, first.y);
                    while let Some(point) = points.next() {
                        ctx.line_to(point.x, point.y);
                    }
                }
                ctx.stroke();
                ctx.set_global_composite_operation("source-over");
            }
            Tool::Line => {
                if stroke.points.len() >= 2 {
                    let start = &stroke.points[0];
                    let end = &stroke.points[stroke.points.len() - 1];
                    ctx.move_to(start.x, start.y);
                    ctx.line_to(end.x, end.y);
                    ctx.stroke();
                }
            }
            Tool::Rectangle => {
                if stroke.points.len() >= 2 {
                    let start = &stroke.points[0];
                    let end = &stroke.points[stroke.points.len() - 1];
                    let x = start.x.min(end.x);
                    let y = start.y.min(end.y);
                    let w = (end.x - start.x).abs();
                    let h = (end.y - start.y).abs();
                    ctx.stroke_rect(x, y, w, h);
                }
            }
            Tool::Circle => {
                if stroke.points.len() >= 2 {
                    let center = &stroke.points[0];
                    let edge = &stroke.points[stroke.points.len() - 1];
                    let radius = ((edge.x - center.x).powi(2) + (edge.y - center.y).powi(2)).sqrt();
                    ctx.begin_path();
                    ctx.arc(center.x, center.y, radius, 0.0, std::f64::consts::PI * 2.0);
                    ctx.stroke();
                }
            }
            Tool::Text => {
                // TODO: Implement text input
            }
        }
    };

    let redraw_all = move || {
        if let Some(ctx) = get_context() {
            let _ = ctx.clear_rect(0.0, 0.0, width as f64, height as f64);

            // Apply pan/zoom transformation
            ctx.save();
            ctx.translate(offset_x.get(), offset_y.get());
            ctx.scale(zoom.get(), zoom.get());

            // Draw all strokes
            for stroke in strokes.get().iter() {
                draw_stroke(&ctx, stroke);
            }

            // Draw current stroke
            if !current_stroke.get().is_empty() {
                let temp_stroke = Stroke {
                    points: current_stroke.get(),
                    color: current_color.get(),
                    thickness: thickness.get(),
                    tool: current_tool.get(),
                };
                draw_stroke(&ctx, &temp_stroke);
            }

            ctx.restore();
        }
    };

    // =====================
    // MOUSE EVENT HANDLERS
    // =====================

    let on_mouse_down = move |ev: MouseEvent| {
        let rect = match canvas_ref.get() {
            Some(canvas) => {
                let web_elem = canvas.into_any();
                let element: Element = match web_elem.dyn_ref::<Element>() {
                    Some(e) => e.clone(),
                    None => return,
                };
                element.get_bounding_client_rect()
            },
            None => return,
        };

        let x = (ev.client_x() as f64 - rect.left()) / zoom.get() - offset_x.get();
        let y = (ev.client_y() as f64 - rect.top()) / zoom.get() - offset_y.get();

        set_is_drawing.set(true);
        set_current_stroke.set(vec![StrokePoint { x, y }]);
    };

    let on_mouse_move = move |ev: MouseEvent| {
        if !is_drawing.get() {
            return;
        }

        let rect = match canvas_ref.get() {
            Some(canvas) => {
                let web_elem = canvas.into_any();
                let element: Element = match web_elem.dyn_ref::<Element>() {
                    Some(e) => e.clone(),
                    None => return,
                };
                element.get_bounding_client_rect()
            },
            None => return,
        };

        let x = (ev.client_x() as f64 - rect.left()) / zoom.get() - offset_x.get();
        let y = (ev.client_y() as f64 - rect.top()) / zoom.get() - offset_y.get();

        let mut points = current_stroke.get();
        points.push(StrokePoint { x, y });
        set_current_stroke.set(points);

        // Redraw for immediate feedback
        redraw_all();
    };

    let on_mouse_up = move |_: MouseEvent| {
        if !is_drawing.get() {
            return;
        }

        set_is_drawing.set(false);

        // Save the stroke
        let stroke = Stroke {
            points: current_stroke.get(),
            color: current_color.get(),
            thickness: thickness.get(),
            tool: current_tool.get(),
        };

        let mut all_strokes = strokes.get();
        all_strokes.push(stroke);
        set_strokes.set(all_strokes);

        // Clear current stroke
        set_current_stroke.set(Vec::new());
        set_undo_stack.set(Vec::new()); // Clear redo stack on new action
        redraw_all();
    };

    // =====================
    // CONTEXT MENU HANDLER
    // =====================

    let on_context_menu = move |ev: MouseEvent| {
        ev.prevent_default();

        // Position context menu at mouse coordinates
        let x = ev.client_x() as f64;
        let y = ev.client_y() as f64;
        set_context_menu_x.set(x);
        set_context_menu_y.set(y);
        set_show_context_menu.set(true);
    };

    let summon_dragrok = move |_| {
        set_show_context_menu.set(false);
        set_dragrok_summoned.set(true);

        // Spawn initial burst of particles
        let mut initial_particles = particles.get();
        for _ in 0..50 {
            initial_particles.push(create_fire_particle(width as f64 / 2.0, height as f64 / 2.0));
        }
        set_particles.set(initial_particles);

        leptos::logging::log!("🐉 DRAGROK SUMMONED WITH FIRE BREATH!");

        // Start animation loop using gloo-timers
        let set_particles_anim = set_particles.clone();
        let particles_anim = particles.clone();

        let _interval = gloo_timers::callback::Interval::new(16, move || {
            let mut current_particles = particles_anim.get();
            let center_x = width as f64 / 2.0;
            let center_y = height as f64 / 2.0;

            // Add new particles (emitter)
            for _ in 0..5 {
                current_particles.push(create_fire_particle(center_x, center_y));
            }

            // Update existing particles
            current_particles.retain_mut(|p| {
                p.x += p.vx;
                p.y += p.vy;
                p.vy -= 0.05; // Upward drift
                p.life -= 1.0;
                p.size *= 0.98; // Shrink

                p.life > 0.0 && p.size > 0.5
            });

            set_particles_anim.set(current_particles);
        });
    };

    fn create_fire_particle(x: f64, y: f64) -> FireParticle {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let angle = rng.gen_range(0.0..2.0 * std::f64::consts::PI);
        let speed = rng.gen_range(1.0..4.0);

        FireParticle {
            x,
            y,
            vx: angle.cos() * speed,
            vy: angle.sin() * speed - 2.0, // Initial upward burst
            life: rng.gen_range(30.0..60.0),
            max_life: 60.0,
            size: rng.gen_range(10.0..25.0),
            color: generate_fire_color(),
        }
    }

    fn generate_fire_color() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let choice = rng.gen_range(0..3);

        match choice {
            0 => "#ffff00", // Yellow (hot)
            1 => "#ff6600", // Orange (medium)
            _ => "#ff0000", // Red (cool)
        }.to_string()
    }

    let close_context_menu = move |_| {
        set_show_context_menu.set(false);
    };

    // =====================
    // KEYBOARD SHORTCUTS
    // =====================

    let on_key_down = move |ev: KeyboardEvent| {
        match ev.key().as_str() {
            "z" if ev.ctrl_key() || ev.meta_key() => {
                // Undo
                let mut all_strokes = strokes.get();
                if let Some(last) = all_strokes.pop() {
                    let mut redo = undo_stack.get();
                    redo.push(last);
                    set_undo_stack.set(redo);
                    set_strokes.set(all_strokes);
                    redraw_all();
                }
            }
            "y" if ev.ctrl_key() || ev.meta_key() => {
                // Redo
                let mut redo = undo_stack.get();
                if let Some(stroke) = redo.pop() {
                    let mut all_strokes = strokes.get();
                    all_strokes.push(stroke);
                    set_strokes.set(all_strokes);
                    set_undo_stack.set(redo);
                    redraw_all();
                }
            }
            "+" | "=" => {
                // Zoom in
                set_zoom.update(|z| *z = (*z * 1.1).min(5.0));
                redraw_all();
            }
            "-" | "_" => {
                // Zoom out
                set_zoom.update(|z| *z = (*z / 1.1).max(0.2));
                redraw_all();
            }
            "0" => {
                // Reset zoom
                set_zoom.set(1.0);
                set_offset_x.set(0.0);
                set_offset_y.set(0.0);
                redraw_all();
            }
            _ => {}
        }
    };

    // =====================
    // WHEEL HANDLER (PAN)
    // =====================

    let on_wheel = move |ev: WheelEvent| {
        if ev.shift_key() {
            // Horizontal pan
            let delta = ev.delta_x() as f64;
            set_offset_x.update(|x| *x -= delta);
        } else {
            // Vertical pan
            let delta = ev.delta_y() as f64;
            set_offset_y.update(|y| *y -= delta);
        }
        ev.prevent_default();
        redraw_all();
    };

    // =====================
    // INITIALIZE CANVAS AFTER MOUNT
    // =====================

    let on_mount = move || {
        // Initialize canvas size
        if let Some(canvas) = canvas_ref.get() {
            let web_canvas = canvas.into_any();
            if let Some(html_canvas) = web_canvas.dyn_ref::<HtmlCanvasElement>() {
                html_canvas.set_width(width);
                html_canvas.set_height(height);

                if let Some(ctx) = get_context() {
                    ctx.set_line_cap("round");
                    ctx.set_line_join("round");
                }
            }
        }
    };

    view! {
        <Style>{r#"
            .whiteboard-container {
                font-family: 'SF Mono', 'Fira Code', 'JetBrains Mono', monospace;
            }

            .context-menu {
                position: absolute;
                background: linear-gradient(135deg, rgba(20, 20, 20, 0.98), rgba(10, 10, 10, 0.99));
                border: 2px solid #ff6b35;
                border-radius: 8px;
                padding: 8px 0;
                min-width: 180px;
                box-shadow: 0 8px 32px rgba(255, 107, 53, 0.4);
                z-index: 10000;
                opacity: 0;
                pointer-events: none;
                transition: opacity 0.2s;
            }

            .context-menu.visible {
                opacity: 1;
                pointer-events: auto;
            }

            .context-menu-item {
                padding: 10px 16px;
                color: #fff;
                font-size: 13px;
                font-weight: 600;
                cursor: pointer;
                transition: background 0.2s;
            }

            .context-menu-item:hover {
                background: rgba(255, 107, 53, 0.2);
            }

            .dragrok-overlay {
                position: absolute;
                top: 0;
                left: 0;
                width: 100%;
                height: 100%;
                background: rgba(0, 0, 0, 0.8);
                display: flex;
                justify-content: center;
                align-items: center;
                z-index: 9999;
                opacity: 0;
                pointer-events: none;
                transition: opacity 0.5s;
            }

            .dragrok-overlay.visible {
                opacity: 1;
                pointer-events: auto;
            }

            .dragrok-container {
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                width: 100%;
                height: 100%;
            }

            .dragrok-title {
                font-size: 48px;
                font-weight: 900;
                color: #ff6b35;
                text-align: center;
                animation: shake 0.5s ease-in-out infinite;
                margin-bottom: 20px;
                z-index: 10001;
            }

            .fire-particle-canvas {
                position: absolute;
                top: 0;
                left: 0;
                width: 100% !important;
                height: 100% !important;
                pointer-events: none;
                z-index: 10000;
            }

            .particle-count {
                position: absolute;
                bottom: 40px;
                font-size: 18px;
                font-weight: 700;
                color: #ff6b35;
                background: rgba(0, 0, 0, 0.7);
                padding: 10px 20px;
                border-radius: 8px;
                border: 2px solid #ff6b35;
                z-index: 10001;
            }

            @keyframes shake {
                0%, 100% { transform: translateX(0); }
                25% { transform: translateX(-10px) rotate(-5deg); }
                75% { transform: translateX(10px) rotate(5deg); }
            }

            @keyframes fire-breath {
                0% { transform: scale(1); opacity: 0.8; }
                100% { transform: scale(1.2); opacity: 1; }
            }
        "#}</Style>

        <div class="whiteboard-container"
            style=format!("width: {}px; height: {}px; position: relative; overflow: hidden; background: #0a0a0a; border: 1px solid #333;", width, height)
        >
            <canvas
                ref=canvas_ref
                class="whiteboard-canvas"
                style="cursor: crosshair; display: block;"
                on:mousedown=on_mouse_down
                on:mousemove=on_mouse_move
                on:mouseup=on_mouse_up
                on:mouseleave=on_mouse_up
                on:wheel=on_wheel
                on:contextmenu=on_context_menu
                on:keydown=on_key_down
                tabindex="0"
            ></canvas>

            // Toolbar
            <div class="whiteboard-toolbar">
                <button
                    class="tool-btn"
                    class:active=move || current_tool.get() == Tool::Pen
                    on:click=move |_| set_current_tool.set(Tool::Pen)
                >
                    "✏️ Pen"
                </button>
                <button
                    class="tool-btn"
                    class:active=move || current_tool.get() == Tool::Eraser
                    on:click=move |_| set_current_tool.set(Tool::Eraser)
                >
                    "🧹 Eraser"
                </button>
                <button
                    class="tool-btn"
                    class:active=move || current_tool.get() == Tool::Line
                    on:click=move |_| set_current_tool.set(Tool::Line)
                >
                    "📏 Line"
                </button>
                <button
                    class="tool-btn"
                    class:active=move || current_tool.get() == Tool::Rectangle
                    on:click=move |_| set_current_tool.set(Tool::Rectangle)
                >
                    "⬜ Rectangle"
                </button>
                <button
                    class="tool-btn"
                    class:active=move || current_tool.get() == Tool::Circle
                    on:click=move |_| set_current_tool.set(Tool::Circle)
                >
                    "⭕ Circle"
                </button>

                <div class="toolbar-divider"></div>

                <button
                    class="color-btn"
                    style=format!("background-color: {};", current_color.get())
                    on:click=move |_| {
                        match current_color.get() {
                            Color::Black => set_current_color.set(Color::Red),
                            Color::Red => set_current_color.set(Color::Green),
                            Color::Green => set_current_color.set(Color::Blue),
                            Color::Blue => set_current_color.set(Color::Yellow),
                            Color::Yellow => set_current_color.set(Color::Cyan),
                            Color::Cyan => set_current_color.set(Color::Magenta),
                            Color::Magenta => set_current_color.set(Color::White),
                            Color::White => set_current_color.set(Color::Black),
                            _ => {}
                        }
                    }
                >
                    "🎨"
                </button>

                <div class="toolbar-divider"></div>

                <button
                    class="action-btn"
                    on:click=move |_| {
                        // Undo
                        let mut all_strokes = strokes.get();
                        if let Some(last) = all_strokes.pop() {
                            let mut redo = undo_stack.get();
                            redo.push(last);
                            set_undo_stack.set(redo);
                            set_strokes.set(all_strokes);
                            redraw_all();
                        }
                    }
                >
                    "↶️ Undo"
                </button>

                <button
                    class="action-btn"
                    on:click=move |_| {
                        // Clear
                        set_strokes.set(Vec::new());
                        set_undo_stack.set(Vec::new());
                        redraw_all();
                    }
                >
                    "🗑️ Clear"
                </button>
            </div>

            // Context Menu (Right-click)
            <div
                class="context-menu"
                class:visible=move || show_context_menu.get()
                style=format!("left: {}px; top: {}px;", context_menu_x.get(), context_menu_y.get())
                on:click=close_context_menu
            >
                <div class="context-menu-item" on:click=summon_dragrok>
                    "🐉 Summon Dragrok"
                </div>
            </div>

            // Dragrok Summoned Visual with Fire Particles
            <div
                class="dragrok-overlay"
                class:visible=move || dragrok_summoned.get()
            >
                <div class="dragrok-container">
                    <div class="dragrok-title">"🐉 DRAGROK APPEARS!"</div>
                    <canvas
                        class="fire-particle-canvas"
                        style=format!("width: {}px; height: {}px;", width, height)
                    ></canvas>
                    <div class="particle-count">
                        {move || {
                            let count = particles.get().len();
                            format!("🔥 {} Fire Particles", count)
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}
