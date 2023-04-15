use web_sys::HtmlInputElement;
use yew::prelude::*;
use stylist::yew::styled_component;
use stylist::style;


#[function_component(App)]
fn app() -> Html {
    let default_output = "NA";
    let string_type = String::from(default_output);
    // Defining output for getallappointments with initial value NA
    let output = use_state(|| string_type);
    // Defining the taskes to perform when button is clicked
    let onclick = {
        let output = output.clone();

        Callback::from(move |_| {
            // call gRPC client for "getallappointments" and returned output can be stored in value
            // for example we take some value
            let value = "Output for getallappointments";
            let value_string = String::from(value);

            output.set(value_string);
        })
    };
    let input_node_ref = use_node_ref();
    let string_type1 = String::from(default_output);
    // Defining output for getuser with initial value NA
    let output_getuser = use_state(|| string_type1);
    // Defining the tasks to perform with the entered input
    let onchange = {
        let input_node_ref = input_node_ref.clone();
        let output_getuser = output_getuser.clone();

        Callback::from(move |_| {
            if let Some(input) = input_node_ref.cast::<HtmlInputElement>() {
                let user_id = input.value();
                // Pass the user_id in gRPC request and store the response in value1
                let def_str = "Output for user ID:";
                let value1 = def_str.to_owned() + &user_id;
                let string_value1 = String::from(value1);

                output_getuser.set(string_value1);
            }
        })
    };
    // macro html for the webpage
    html! {
        <>
            <label for="getallappointments">
                { "Get information about all appointments stored in the tower : " }
                <button
                    class="button-77"
                    {onclick}>{ "getallappointments" }
                </button>
            </label>

            <p>{ &*output }</p>

            <label for="getuser">
               { "Get information about a specific user : " }
               <input ref={input_node_ref}
                   {onchange}
                   id="my-input"
                   type="text"
                   placeholder="User ID"
               />
           </label>
           
            <p>{ &*output_getuser }</p>

            <label for="selection">
                {"Select menu for some command (if required) : "}
                <select name="select">
                    <option value="" disabled=true selected=true hidden=true>{"Please Choose..."}</option>
                    <option selected=true disabled=false value="">{ "Can be selected" }</option>
                    <option selected=false disabled=true value="">{ "Can't be selected" }</option>
                </select>
            </label>

            <p>{". . . Similarly, more functions can be added."}</p>
            <p>{"By Ayush Ranjan, for Summer of Bitcoin 2023!"}</p>
        </>
    }
}

// CSS Implementation (We can also implement it in separate file)
#[styled_component(App1)]
pub fn app1() -> Html {
    let stylesheet = style!(
        r#"
            p{
                font-family: Verdana, sans-serif;
                font-size: 30px;
                color: orange;
              } 
            button {
                font-family: Verdana, sans-serif;
                padding: 15px 32px;
                text-align: center;
                text-decoration: none;
                display: inline-block;
                font-size: 16px;
                margin: 4px 2px;
                cursor: pointer;
                background-color: white; 
                color: black; 
                border: 2px solid brown;
                transition-duration: 0.4s;
              }
              
            button:hover {
                background-color: brown; /* Green */
                color: white;
            }
            input {
                transition: width 0.4s ease-in-out;
                margin: 4px 2px;
                border: 2px solid brown;
            }
            input:focus {
                width: 25%;
            }
            select {
                width: 30%;
                padding: 16px 20px;
                border-radius: 4px;
                margin: 4px 2px;
                border: 2px solid brown;
            }
        "#
    )
    .unwrap();
    let logo=style!(
        r#"
            display: flex;
            justify-content: center;
            align-items: center;
            img {           
                max-width: 10%;
                max-height: 10%;
              }
        "#
    ).unwrap();
    let title=style!(
        r#"
            font-family: Verdana, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            color: brown;
            font-size: 30px;
        "#
    ).unwrap();
    html! {
        <>
            <link rel="icon" type="image/x-icon" href="https://www.talaia.watch/img/logo.jpg"/>
            <div class={logo}>
                <img src="https://pbs.twimg.com/profile_images/1248211901571457027/c4-cYPSS_400x400.jpg" alt="error in displaying logo"/> 
            </div>
            <div class={title}>
                <p>{"Web-based GUI for teosd"}</p>
            </div>
            <div class={stylesheet}>
                <App/>
                
            </div>
            
        </>
    }
}
fn main() {
    yew::Renderer::<App1>::new().render();
}
