#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::{Address as _},
    token, Address, Env, String, Vec,
};

fn create_token<'a>(env: &Env, admin: &Address) -> token::StellarAssetClient<'a> {
    let token = token::StellarAssetClient::new(env, &env.register_stellar_asset_contract_v2(admin.clone()).address());
    token.mint(admin, &10_000);
    token
}

#[test]
fn test_create_and_apply() {
    let env = Env::default();
    env.mock_all_auths();

    let poster = Address::generate(&env);
    let reviewer = Address::generate(&env);
    let applicant = Address::generate(&env);

    let token = create_token(&env, &poster);
    let token_address = token.address.clone();

    let contract_id = env.register(GrantBoard, ());
    let client = GrantBoardClient::new(&env, &contract_id);

    let mut milestones = Vec::new(&env);
    milestones.push_back(Milestone {
        description: String::from_str(&env, "First milestone"),
        amount: 500,
        approved: false,
    });
    milestones.push_back(Milestone {
        description: String::from_str(&env, "Second milestone"),
        amount: 500,
        approved: false,
    });

    let grant_id = client.create_grant(
        &poster,
        &reviewer,
        &String::from_str(&env, "Test Grant"),
        &String::from_str(&env, "A test grant"),
        &token_address,
        &1000,
        &milestones,
    );

    assert_eq!(grant_id, 1);

    client.apply(&grant_id, &applicant);

    let grant = client.get_grant(&grant_id);
    assert_eq!(grant.applicants.len(), 1);
}

#[test]
fn test_select_and_approve() {
    let env = Env::default();
    env.mock_all_auths();

    let poster = Address::generate(&env);
    let reviewer = Address::generate(&env);
    let applicant = Address::generate(&env);

    let token = create_token(&env, &poster);
    let token_address = token.address.clone();

    let contract_id = env.register(GrantBoard, ());
    let client = GrantBoardClient::new(&env, &contract_id);

    let mut milestones = Vec::new(&env);
    milestones.push_back(Milestone {
        description: String::from_str(&env, "Milestone 1"),
        amount: 1000,
        approved: false,
    });

    let grant_id = client.create_grant(
        &poster,
        &reviewer,
        &String::from_str(&env, "Grant"),
        &String::from_str(&env, "Desc"),
        &token_address,
        &1000,
        &milestones,
    );

    client.apply(&grant_id, &applicant);
    client.select_applicant(&grant_id, &applicant);
    client.approve_milestone(&grant_id);

    let grant = client.get_grant(&grant_id);
    assert!(grant.milestones.get(0).unwrap().approved);
}