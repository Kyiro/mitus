import register from "preact-custom-element";

function component() {
    return (
        <p>This is an example for "Mitus"</p>
    );
}

register(component, "x-component", []);