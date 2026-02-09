# Idea: LiveView-like Frontend via WASM

## Concept

LiveView-style systems let you write interactive web apps with a simple model:

* keep application state in one place (a `Model`)
* render the UI with a pure `view : Model -> Html`
* handle events with `update : Msg -> Model -> (Model, Effect)`

AIVI can adopt the same *programming model* but **run the logic locally in the browser via WASM**, producing DOM patches without a large JS framework.

## Architecture

1.  **Model**: An immutable record representing the UI state.
2.  **View**: A pure function `Model -> Html` (via the `Html` domain + JSX literals).
3.  **Update**: A pure function `Msg -> Model -> (Model, Effect)`.

## The "Live" Part

Instead of a Virtual DOM diffing in JS, or a server-side diff:

1.  AIVI code produces a DOM tree description (using HTML Domain).
2.  The AIVI runtime (WASM) computes the diff against the previous version.
3.  WASM calls into JS only to apply the minimal patch.

This creates a **"No-JS" experience** for the developer. They write only AIVI code.

## Why AIVI fits this

*   **Generators**: Perfect for handling streams of events (clicks, inputs).
*   **Patching (`<=`)**: The deep update syntax makes state management trivial (Redux/Zustand replacement built-in).
*   **Domains**: HTML structure is typed via `Html`, and styling is typed via `Style` (CSS unit deltas like `16px`, `100%`, `100svh`, `5cqw`).

## Example

```aivi
use aivi.std.html
use aivi.std.style

Model = { count: Int }
Msg = Inc | Dec

update : Msg -> Model -> Model
update msg model =
  msg ?
  | Inc => model <= { count: _ + 1 }
  | Dec => model <= { count: _ - 1 }

containerStyle : StyleSheet
containerStyle = [
  ("padding", 16px)
  ("minHeight", 100svh)
]

buttonStyle : StyleSheet
buttonStyle = [
  ("padding", 10px)
  ("borderRadius", 8px)
  ("minWidth", 48px)
]

css : StyleSheet -> Text
css sheet = renderInlineCss sheet

view : Model -> Element
view model = <div style={css containerStyle}>
  <button onClick={Dec} style={css buttonStyle}>-</button>
  <span>Count: {model.count}</span>
  <button onClick={Inc} style={css buttonStyle}>+</button>
</div>
```
