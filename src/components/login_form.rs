use stylist::yew::styled_component;
use yew::{html, Html};

#[styled_component(LoginForm)]
pub fn login_form() -> Html {
    html! {
        <div class={css!("
            width: 100vw;
            height: 100vh;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            background-color: #225522;
        ")}>
            <div class={css!("
                width: 400px;
                height: 400px;
                background-color: #336633;
                padding: 25px;
            ")}>
                <form class={css!("
                    height: 100%;
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: flex-end;
                ")}>
                <label class={css!("
                    width: 100%;
                    display: flex;
                    flex-direction: column;
                    margin-block-end: 30px;
                    color: white;
                ")}>
                    { "Username" }
                    <input class={css!("
                        width: 100%;
                        background-color: #336633;
                        border: none;
                        border-bottom: 2px solid white;
                        padding: 5px 0;
                        color: white;
                        font-size: 18px;
                    ")} type="text" name="login" placeholder="armin@sepmin.com" />
                </label>
                <label class={css!("
                    width: 100%;
                    display: flex;
                    flex-direction: column;
                    margin-block-end: 30px;
                    color: white;
                ")}>
                    { "Password" }
                    <input class={css!("
                        width: 100%;
                        background-color: #336633;
                        border: none;
                        border-bottom: 2px solid white;
                        padding: 5px 0;
                        color: white;
                        font-size: 18px;
                    ")} type="password" name="password" />
                </label>
                <button class={css!("
                    width: 80px;
                    height: 30px;
                    background-color: white;
                    color: #121212;
                    border: none;
                ")} type="submit">{"Sign in"}</button>
            </form>
            </div>
        </div>
    }
}