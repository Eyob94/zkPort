use leptos::prelude::*;
use thaw::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <ConfigProvider>
            <Button
                appearance=ButtonAppearance::Primary
                on:click=move |_| {
                    *set_count.write() += 1;
                }
            >

        Increment
            </Button>
            <div class="text-red-500 text-3xl">
                {count}
            </div>
        </ConfigProvider>
    }
}
