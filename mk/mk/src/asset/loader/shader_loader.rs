use crate::{asset::AssetLoader, handles::*};
use std::fs::read_to_string;

pub fn shader_loader() -> AssetLoader<ShaderHandle> {
    AssetLoader::new(|context, base, path| {
        let path = path.with_extension("wgsl");
        let shader = read_to_string(base.join("shaders").join(path))?;
        let shader = context.render_mgr().create_shader(shader);

        // let (shader, vertex_shader_log, fragment_shader_log, log) =
        //     ShaderHandle::from_source(&vs, &fs);
        // let shader = match shader {
        //     Ok(shader) => {
        //         if vertex_shader_log.is_some() || fragment_shader_log.is_some() || log.is_some() {
        //             emit_diagnostic_info!(
        //                 format!("while loading asset {}...", type_name::<ShaderHandle>()),
        //                 vec![
        //                     vertex_shader_log.map(|log| subdiag_warn!(format!(
        //                         "log output while compiling vertex shader:\n{}",
        //                         log
        //                     ))),
        //                     fragment_shader_log.map(|log| subdiag_warn!(format!(
        //                         "log output while compiling fragment shader:\n{}",
        //                         log
        //                     ))),
        //                     log.map(|log| subdiag_warn!(format!(
        //                         "log output while linking shader program:\n{}",
        //                         log
        //                     ))),
        //                 ]
        //                 .into_iter()
        //                 .filter_map(|sub_diagnostic| sub_diagnostic)
        //                 .collect()
        //             );
        //         }

        //         shader
        //     }
        //     Err(err) => {
        //         if vertex_shader_log.is_some() || fragment_shader_log.is_some() || log.is_some() {
        //             emit_diagnostic_error!(
        //                 format!(
        //                     "error detected while loading asset {}: {}",
        //                     type_name::<ShaderHandle>(),
        //                     err
        //                 ),
        //                 vec![
        //                     vertex_shader_log.map(|log| subdiag_warn!(format!(
        //                         "log output while compiling vertex shader:\n{}",
        //                         log
        //                     ))),
        //                     fragment_shader_log.map(|log| subdiag_warn!(format!(
        //                         "log output while compiling fragment shader:\n{}",
        //                         log
        //                     ))),
        //                     log.map(|log| subdiag_warn!(format!(
        //                         "log output while linking shader program:\n{}",
        //                         log
        //                     ))),
        //                 ]
        //                 .into_iter()
        //                 .filter_map(|sub_diagnostic| sub_diagnostic)
        //                 .collect()
        //             );
        //         } else {
        //             emit_diagnostic_error!(format!(
        //                 "error detected while loading {}: {}",
        //                 type_name::<ShaderHandle>(),
        //                 err
        //             ));
        //         }

        //         return Err(AssetLoadError::other(err));
        //     }
        // };

        Ok(shader)
    })
}
