#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, Env, String, Vec,
};

#[contracttype]
#[derive(Clone)]
pub struct Milestone {
    pub description: String,
    pub amount: i128,
    pub approved: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct Grant {
    pub id: u64,
    pub poster: Address,
    pub reviewer: Address,
    pub title: String,
    pub description: String,
    pub token: Address,
    pub total_amount: i128,
    pub milestones: Vec<Milestone>,
    pub applicants: Vec<Address>,
    pub selected: Option<Address>,
    pub current_milestone: u32,
}

#[contracttype]
pub enum DataKey {
    Grant(u64),
    GrantCount,
}

#[contract]
pub struct GrantBoard;

#[contractimpl]
impl GrantBoard {
    pub fn create_grant(
        env: Env,
        poster: Address,
        reviewer: Address,
        title: String,
        description: String,
        token: Address,
        total_amount: i128,
        milestones: Vec<Milestone>,
    ) -> u64 {
        poster.require_auth();

        // Transfer funds into the contract
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&poster, &env.current_contract_address(), &total_amount);

        let count: u64 = env.storage().instance().get(&DataKey::GrantCount).unwrap_or(0);
        let id = count + 1;

        let grant = Grant {
            id,
            poster,
            reviewer,
            title,
            description,
            token,
            total_amount,
            milestones,
            applicants: Vec::new(&env),
            selected: None,
            current_milestone: 0,
        };

        env.storage().instance().set(&DataKey::Grant(id), &grant);
        env.storage().instance().set(&DataKey::GrantCount, &id);

        id
    }

    pub fn apply(env: Env, grant_id: u64, applicant: Address) {
        applicant.require_auth();

        let mut grant: Grant = env.storage().instance().get(&DataKey::Grant(grant_id)).unwrap();

        assert!(grant.selected.is_none(), "Applicant already selected");
        assert!(!grant.applicants.contains(&applicant), "Already applied");

        grant.applicants.push_back(applicant);
        env.storage().instance().set(&DataKey::Grant(grant_id), &grant);
    }

    pub fn select_applicant(env: Env, grant_id: u64, applicant: Address) {
        let mut grant: Grant = env.storage().instance().get(&DataKey::Grant(grant_id)).unwrap();

        grant.reviewer.require_auth();
        assert!(grant.selected.is_none(), "Already selected");
        assert!(grant.applicants.contains(&applicant), "Not an applicant");

        grant.selected = Some(applicant);
        env.storage().instance().set(&DataKey::Grant(grant_id), &grant);
    }

    pub fn approve_milestone(env: Env, grant_id: u64) {
        let mut grant: Grant = env.storage().instance().get(&DataKey::Grant(grant_id)).unwrap();

        grant.reviewer.require_auth();

        let idx = grant.current_milestone as usize;
        assert!(idx < grant.milestones.len() as usize, "All milestones done");

        let mut milestone = grant.milestones.get(idx as u32).unwrap();
        assert!(!milestone.approved, "Already approved");

        let recipient = grant.selected.clone().expect("No applicant selected");
        let token_client = token::Client::new(&env, &grant.token);
        token_client.transfer(&env.current_contract_address(), &recipient, &milestone.amount);

        milestone.approved = true;
        grant.milestones.set(idx as u32, milestone);
        grant.current_milestone += 1;

        env.storage().instance().set(&DataKey::Grant(grant_id), &grant);
    }

    pub fn get_grant(env: Env, grant_id: u64) -> Grant {
        env.storage().instance().get(&DataKey::Grant(grant_id)).unwrap()
    }
}
mod test;