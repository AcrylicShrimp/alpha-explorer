use super::Vec3;
use crate::script::api::LuaApiTable;
use mlua::prelude::*;

pub type Mat33 = crate::structure::Mat33;

// impl LuaApiTable for Mat33 {
//     fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
//         let table = lua.create_table()?;

//         table.set(
//             "new",
//             lua.create_function(|_lua, (e00, e01, e02, e03, e04, e05, e06, e07, e08)| {
//                 Ok(Self::new([e00, e01, e02, e03, e04, e05, e06, e07, e08]))
//             })?,
//         )?;
//         table.set("zero", lua.create_function(|_lua, ()| Ok(Self::zero()))?)?;
//         table.set(
//             "identity",
//             lua.create_function(|_lua, ()| Ok(Self::identity()))?,
//         )?;
//         table.set(
//             "affine_translation",
//             lua.create_function(|_lua, t| Ok(Self::affine_translation(t)))?,
//         )?;
//         table.set(
//             "affine_rotation",
//             lua.create_function(|_lua, angle_degrees| Ok(Self::affine_rotation(angle_degrees)))?,
//         )?;
//         table.set(
//             "affine_scale",
//             lua.create_function(|_lua, s| Ok(Self::affine_scale(s)))?,
//         )?;
//         table.set(
//             "affine_srt",
//             lua.create_function(|_lua, (t, angle_degrees, s)| {
//                 Ok(Self::affine_srt(t, angle_degrees, s))
//             })?,
//         )?;
//         table.set(
//             "affine_trs",
//             lua.create_function(|_lua, (t, angle_degrees, s)| {
//                 Ok(Self::affine_trs(t, angle_degrees, s))
//             })?,
//         )?;

//         Ok(table)
//     }
// }

// impl LuaUserData for Mat33 {
//     fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
//         methods.add_meta_method(LuaMetaMethod::ToString, |_lua, this, ()| {
//             Ok(this.to_string())
//         });

//         methods.add_meta_function(LuaMetaMethod::Add, |_lua, (lhs, rhs): (Self, Self)| {
//             Ok(lhs + rhs)
//         });

//         methods.add_meta_function(LuaMetaMethod::Sub, |_lua, (lhs, rhs): (Self, Self)| {
//             Ok(lhs - rhs)
//         });

//         methods.add_meta_function(
//             LuaMetaMethod::Mul,
//             |lua, (lhs, rhs): (LuaValue, LuaValue)| match (&lhs, &rhs) {
//                 (_, &LuaValue::Integer(..)) => {
//                     (Self::from_lua(lhs, lua)? * f32::from_lua(rhs, lua)?).to_lua(lua)
//                 }
//                 (_, &LuaValue::Number(..)) => {
//                     (Self::from_lua(lhs, lua)? * f32::from_lua(rhs, lua)?).to_lua(lua)
//                 }
//                 (&LuaValue::Integer(..), _) => {
//                     (f32::from_lua(lhs, lua)? * Self::from_lua(rhs, lua)?).to_lua(lua)
//                 }
//                 (&LuaValue::Number(..), _) => {
//                     (f32::from_lua(lhs, lua)? * Self::from_lua(rhs, lua)?).to_lua(lua)
//                 }
//                 (_, LuaValue::UserData(rhs_inner)) if rhs_inner.is::<Vec3>() => {
//                     (Self::from_lua(lhs, lua)? * Vec3::from_lua(rhs, lua)?).to_lua(lua)
//                 }
//                 (LuaValue::UserData(lhs_inner), _) if lhs_inner.is::<Vec3>() => {
//                     (Vec3::from_lua(lhs, lua)? * Self::from_lua(rhs, lua)?).to_lua(lua)
//                 }
//                 _ => (Self::from_lua(lhs, lua)? * Self::from_lua(rhs, lua)?).to_lua(lua),
//             },
//         );

//         methods.add_meta_function(LuaMetaMethod::Unm, |_lua, lhs: Self| Ok(-lhs));

//         methods.add_method("elements", |_lua, this, ()| Ok(this.elements().clone()));
//         methods.add_method("row", |_lua, this, index| Ok(this.row(index)));
//         methods.add_method("column", |_lua, this, index| Ok(this.column(index)));
//         methods.add_method("determinant", |_lua, this, ()| Ok(this.determinant()));
//         methods.add_method_mut("inverse", |_lua, this, ()| {
//             this.inverse();
//             Ok(())
//         });
//         methods.add_method("inversed", |_lua, this, ()| Ok(this.inversed()));
//         methods.add_method_mut("transpose", |_lua, this, ()| {
//             this.transpose();
//             Ok(())
//         });
//         methods.add_method("transposed", |_lua, this, ()| Ok(this.transposed()));
//         methods.add_method_mut("element_wise_multiply", |_lua, this, rhs: Self| {
//             this.element_wise_multiply(rhs.as_ref());
//             Ok(())
//         });
//         methods.add_method("element_wise_multiplied", |_lua, this, rhs: Self| {
//             Ok(this.element_wise_multiplied(rhs.as_ref()))
//         });
//         methods.add_method_mut("element_wise_divide", |_lua, this, rhs: Self| {
//             this.element_wise_divide(rhs.as_ref());
//             Ok(())
//         });
//         methods.add_method("element_wise_divided", |_lua, this, rhs: Self| {
//             Ok(this.element_wise_divided(rhs.as_ref()))
//         });
//     }
// }
