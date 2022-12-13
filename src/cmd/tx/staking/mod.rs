mod cancel_leave_candidates;
mod candidate_stake_less;
mod claim_rewards;
mod execute_leave_candidates;
mod execute_scheduled_rewards_change;
mod force_new_round;
mod force_remove_candidate;
mod increment_collator_rewards;
mod init_leave_candidates;
mod join_candidates;
mod join_delegators;
mod leave_delegators;

pub fn command() -> clap::Command {
    clap::Command::new("staking")
        .about("Staking transactions")
        .subcommand_required(true)
        .subcommands([
            candidate_stake_less::command(),
            cancel_leave_candidates::command(),
            claim_rewards::command(),
            execute_leave_candidates::command(),
            execute_scheduled_rewards_change::command(),
            force_new_round::command(),
            force_remove_candidate::command(),
            increment_collator_rewards::command(),
            init_leave_candidates::command(),
            join_candidates::command(),
            join_delegators::command(),
            leave_delegators::command(),
        ])
}

pub async fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("candidate-stake-less", matches)) => candidate_stake_less::run(matches).await,
        Some(("cancel-leave-candidates", matches)) => cancel_leave_candidates::run(matches).await,
        Some(("claim-rewards", matches)) => claim_rewards::run(matches).await,
        Some(("execute-leave-candidates", matches)) => execute_leave_candidates::run(matches).await,
        Some(("execute-scheduled-rewards-change", matches)) => {
            execute_scheduled_rewards_change::run(matches).await
        }
        Some(("force-new-round", matches)) => force_new_round::run(matches).await,
        Some(("force-remove-candidate", matches)) => force_remove_candidate::run(matches).await,
        Some(("increment-collator-rewards", matches)) => {
            increment_collator_rewards::run(matches).await
        }
        Some(("init-leave-candidates", matches)) => init_leave_candidates::run(matches).await,
        Some(("join-candidates", matches)) => join_candidates::run(matches).await,
        Some(("join-delegators", matches)) => join_delegators::run(matches).await,
        Some(("leave-delegators", matches)) => leave_delegators::run(matches).await,
        _ => unreachable!(),
    }
}
