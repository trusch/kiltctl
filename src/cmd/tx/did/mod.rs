mod add_key_agreement_key;
mod add_service_endpoint;
mod authorize;
mod create;
mod delete;
mod remove_attestation_key;
mod remove_delegation_key;
mod remove_key_agreement_key;
mod remove_service_endpoint;
mod set_attestation_key;
mod set_authentication_key;
mod set_delegation_key;

pub fn command() -> clap::Command {
    clap::Command::new("did")
        .about("DID transactions")
        .subcommand_required(true)
        .subcommands([
            create::command(),
            add_key_agreement_key::command(),
            add_service_endpoint::command(),
            delete::command(),
            remove_attestation_key::command(),
            remove_delegation_key::command(),
            remove_key_agreement_key::command(),
            remove_service_endpoint::command(),
            set_attestation_key::command(),
            authorize::command(),
            set_authentication_key::command(),
            set_delegation_key::command(),
        ])
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("create", matches)) => create::run(matches).await,
        Some(("add-key-agreement-key", matches)) => add_key_agreement_key::run(matches).await,
        Some(("add-service-endpoint", matches)) => add_service_endpoint::run(matches).await,
        Some(("delete", matches)) => delete::run(matches).await,
        Some(("remove-attestation-key", matches)) => remove_attestation_key::run(matches).await,
        Some(("remove-delegation-key", matches)) => remove_delegation_key::run(matches).await,
        Some(("remove-key-agreement-key", matches)) => remove_key_agreement_key::run(matches).await,
        Some(("remove-service-endpoint", matches)) => remove_service_endpoint::run(matches).await,
        Some(("set-attestation-key", matches)) => set_attestation_key::run(matches).await,
        Some(("set-authentication-key", matches)) => set_authentication_key::run(matches).await,
        Some(("set-delegation-key", matches)) => set_delegation_key::run(matches).await,
        Some(("authorize", matches)) => authorize::run(matches).await,

        _ => Ok(()),
    }
}
