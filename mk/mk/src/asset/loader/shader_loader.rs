use crate::asset::{AssetLoadError, AssetLoader};
use crate::{emit_diagnostic_error, emit_diagnostic_info, subdiag_warn};
use render::Shader;
use std::any::type_name;
use std::fs::read_to_string;
use std::sync::Arc;

pub fn shader_loader() -> AssetLoader<Shader> {
    AssetLoader::new(|_asset_mgr, base, path| {
        let vs = read_to_string(base.join("shaders").join(path).join("vertex.glsl"))?;
        let fs = read_to_string(base.join("shaders").join(path).join("fragment.glsl"))?;

        let (shader, vertex_shader_log, fragment_shader_log, log) = Shader::from_source(&vs, &fs);
        let shader = match shader {
            Ok(shader) => {
                if vertex_shader_log.is_some() || fragment_shader_log.is_some() || log.is_some() {
                    emit_diagnostic_info!(
                        format!("while loading asset {}...", type_name::<Shader>()),
                        vec![
                            vertex_shader_log.map(|log| subdiag_warn!(format!(
                                "log output while compiling vertex shader:\n{}",
                                log
                            ))),
                            fragment_shader_log.map(|log| subdiag_warn!(format!(
                                "log output while compiling fragment shader:\n{}",
                                log
                            ))),
                            log.map(|log| subdiag_warn!(format!(
                                "log output while linking shader program:\n{}",
                                log
                            ))),
                        ]
                        .into_iter()
                        .filter_map(|sub_diagnostic| sub_diagnostic)
                        .collect()
                    );
                }

                shader
            }
            Err(err) => {
                if vertex_shader_log.is_some() || fragment_shader_log.is_some() || log.is_some() {
                    emit_diagnostic_error!(
                        format!(
                            "error detected while loading asset {}: {}",
                            type_name::<Shader>(),
                            err
                        ),
                        vec![
                            vertex_shader_log.map(|log| subdiag_warn!(format!(
                                "log output while compiling vertex shader:\n{}",
                                log
                            ))),
                            fragment_shader_log.map(|log| subdiag_warn!(format!(
                                "log output while compiling fragment shader:\n{}",
                                log
                            ))),
                            log.map(|log| subdiag_warn!(format!(
                                "log output while linking shader program:\n{}",
                                log
                            ))),
                        ]
                        .into_iter()
                        .filter_map(|sub_diagnostic| sub_diagnostic)
                        .collect()
                    );
                } else {
                    emit_diagnostic_error!(format!(
                        "error detected while loading {}: {}",
                        type_name::<Shader>(),
                        err
                    ));
                }

                return Err(AssetLoadError::other(err));
            }
        };

        Ok(Arc::new(shader))
    })
}
