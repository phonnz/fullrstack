use wasm_bindgen_test::*;
use fullrstack_web::components::dashboard::Dashboard;
use leptos::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_dashboard_render() {
    let runtime = create_runtime();
    
    create_scope(runtime, |cx| {
        // Mount dashboard component
        let dashboard = view! { cx,
            <Dashboard/>
        };
        
        // Assert dashboard elements are present
        assert!(dashboard.into_view().render_to_string().contains("device-table"));
    });
} 