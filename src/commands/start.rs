use bollard::{
    container::{Config, CreateContainerOptions, StartContainerOptions},
    Docker,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub(crate) async fn run() {
    let docker = Docker::connect_with_socket_defaults().unwrap();
    let supabase_project_id = "supapasskeys-cli";
    let database_port = 6432;
    let database_url = format!(
        "postgres://supapasskeys:supapasskeys@supabase_pooler_{}:{}/postgres",
        supabase_project_id, database_port
    );
    let supapasskeys_config = Config {
        image: Some("ghcr.io/jehrhardt/supapasskeys:main".to_string()),
        env: Some(vec![
            format!("DATABASE_URL={}", database_url),
            "DATABASE_AFTER_CONNECT_QUERY=SET search_path TO _supapasskeys;".to_string(),
            format!("SECRET_KEY_BASE={}", generate_secret_key_base()),
            format!("SUPABASE_PROJECT_ID={}", supabase_project_id),
            format!("RELYING_PARTY_NAME={}", supabase_project_id),
            "RELYING_PARTY_ORIGIN=http://localhost:3000".to_string(),
        ]),
        ..Default::default()
    };
    let _ = &docker
        .create_container(
            Some(CreateContainerOptions {
                name: "supapasskeys".to_string(),
                platform: None,
            }),
            supapasskeys_config,
        )
        .await;
    let _ = &docker
        .start_container("supapasskeys", None::<StartContainerOptions<String>>)
        .await;
}

fn generate_secret_key_base() -> String {
    let secret_key_base = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .collect::<Vec<u8>>();
    String::from_utf8(secret_key_base).unwrap()
}
