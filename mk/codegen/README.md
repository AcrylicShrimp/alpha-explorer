# codegen

This sub-crate offers procedural macros to simplify engine code.

## Animation

Components provided and/or used by the `mk` engine is statically compiled. To animate them, we have to inject animating helpers for each component before compilation. We cannot modify memory on runtime to manipulate animated properties because the `Rust` have a "varying" memory model far from the `C` which is concrete. This `animation` macro module will automatically generate animation logics for all of properties of given component.

### Example

```rust
#[derive(Animation)]
pub struct Animated {
    #[animate]
    pub a: i64,
    #[animate]
    pub b: f32,
}
```

## Lua

The `mk` engine uses the `Lua` as a scripting language. This means that all components/structs/functions must be binded into the `Lua` context somehow. But this binding should be carefully designed since they affects performance directly. It is very hard to provide good binding implementations of tons of objects which have diverse characteristics. Fortunately, we can classify them into a few groups by their usage. This `lua` macro module provides binding implementations with powerful features to cover edge-cases.

### Implementation Types

There are some implementations you can choose. They provide difference performance characteristics/drawbacks. Here's a quick comparison table.

| Name                    | Wrapper | What `Lua` Holds               | What `Rust` Holds    |
| ----------------------- | ------- | ------------------------------ | -------------------- |
| `LuaStruct`             | NO      | `Table`                        | `Data`               |
| `LuaRc`                 | YES     | `UserData<Wrapper<Arc<Data>>>` | `Wrapper<Arc<Data>>` |
| `LuaComponent`          | YES     | `UserData<Wrapper<Entity>>`    | `World`              |
| `LuaComponentNoWrapper` | NO      | `UserData<Data>`               | `Context`            |

#### `LuaStruct`

The `LuaStruct` creates a lua table and put all of fields into it. All fields will be copied when put them into the lua and vice-versa.

#### `LuaRc`

The `LuaRc` moves the given data into an `Arc` instance and pass it to the lua. `__index` and `__newindex` meta-method will be provided to access to the rust memory.

### Field Control

You can control fields for various purpose such as excluding, overriding and providing user methods.

Struct-level:

- `#[lua_method(METHOD_NAME)]`: The method will be exposed on lua system.

Field-level:

- `#[lua_hidden]`: The field will not be visible on lua system. This field can be non-lua-compatible.
<!-- - `#[]`:  -->
