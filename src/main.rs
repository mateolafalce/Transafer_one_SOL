use anchor_client::{
    anchor_lang::system_program,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signature},
        signer::Signer,
    },
    Client, Cluster, Program,
};
use anyhow::Result;
use std::rc::Rc;
use std::str::FromStr;

fn main() -> Result<()> {
    let payer = read_keypair_file(&*shellexpand::tilde(
        "C:/Users/Mateo/.config/solana/id.json",
    ))
    .expect("Example requires a keypair file");
    let client: Client = Client::new(Cluster::Devnet, Rc::new(payer));
    let program: Program =
        client.program(Pubkey::from_str("97ico5tgMcM8xyeumNUroM51bKgnjjWgSbVdxqYPJYVb").unwrap());
    transfer(program, "Enjoy a crypto coffe".to_string())?;
    Ok(())
}

fn transfer(program: Program, msg: String) -> Result<()> {
    let luck_addres: Keypair = Keypair::new();
    let signature: Signature = program
        .request()
        .accounts(transfer_one_sol::accounts::Transaction {
            from: program.payer(),
            to: luck_addres.pubkey(),
            system_program: system_program::ID,
        })
        .args(transfer_one_sol::instruction::SendOneSol { msg })
        .send()?;
    println!("{}", signature);
    Ok(())
}
