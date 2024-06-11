#[tokio::main]
async fn main() -> Result<()> {
    let router = Router::new()
        .route(
            "/actions/fixed_transfer",
            get(handle_get_fixed_transfer_action).post(handle_post_fixed_transfer_action),
        )
        .route(
            "/actions/dynamic_transfer",
            get(handle_get_dynamic_transfer_action).post(handle_post_dynamic_transfer_action),
        );

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

pub fn handle_get_fixed_transfer_action() -> Result<Json<ActionMetadata>> {
    Ok(Json(FixedTransferAction::to_metadata()))
}

pub fn handle_post_fixed_transfer_action(
    Json(payload): Json<CreateActionPayload>,
) -> Result<Json<ActionTransaction>> {
    let transaction = FixedTransferAction::create_transaction(Context { payload });
    let serialized_transaction = match serialize(&transaction) {
        Ok(serialized_transaction) => serialized_transaction,
        _ => return Err(Error::InvalidInstruction),
    };
    let encoded_transaction = BASE64_STANDARD.encode(serialized_transaction);

    Ok(Json(ActionTransaction {
        transaction: encoded_transaction,
        message: None
    }))
}

impl CreateTransaction for FixedTransferAction {
    fn create_transaction(ctx: Context<FixedTransferAction>) {
        let account_pubkey = match Pubkey::from_str(&ctx.payload.account) {
            Ok(account_pubkey) => account_pubkey,
            _ => return Err(Error::InvalidAccountPubkey),
        };
        let mint_pubkey =
            Pubkey::from_str(&"4PYnraBJbdPXeMXdgL5k1m3TCcfNMaEWycvEQu2cteEV").unwrap();
        let receiver_pubkey =
            Pubkey::from_str(&"6GBLiSwAPhDMttmdjo3wvEsssEnCiW3yZwVyVZnhFm3G").unwrap();
        let source_pubkey = spl_associated_token_account::get_associated_token_address(
            &account_pubkey,
            &mint_pubkey,
        );
        let destination_pubkey = spl_associated_token_account::get_associated_token_address(
            &receiver_pubkey,
            &mint_pubkey,
        );
        let transfer_instruction = match spl_token::instruction::transfer(
            &spl_token::ID,
            &source_pubkey,
            &destination_pubkey,
            &account_pubkey,
            &[&account_pubkey],
            1,
        ) {
            Ok(transfer_instruction) => transfer_instruction,
            _ => return Err(Error::InvalidInstruction),
        };
        let transaction_message = Message::new(&[transfer_instruction], None);

        Ok(Transaction::new_unsigned(transaction_message))
    }
}


pub fn handle_get_dynamic_transfer_action() -> Result<Json<ActionMetadata>> {
    Ok(Json(DynamicTransferAction::to_metadata()))
}

pub fn handle_post_dynamic_transfer_action(
    Json(payload): Json<CreateActionPayload>,
    Query(query): Query<DynamicTransferQuery>,
) -> Result<Json<ActionTransaction>> {
    let transaction = DynamicTransferAction::create_transaction(Context { payload, query });
    let serialized_transaction = match serialize(&transaction) {
        Ok(serialized_transaction) => serialized_transaction,
        _ => return Err(Error::InvalidInstruction),
    };
    let encoded_transaction = BASE64_STANDARD.encode(serialized_transaction);

    Ok(Json(ActionTransaction {
        transaction: encoded_transaction,
        message: None
    }))
}

impl CreateTransaction for DynamicTransferAction {
    fn create_transaction(ctx: Context<DynamicTransferAction>) {
        let account_pubkey = match Pubkey::from_str(&ctx.payload.account) {
            Ok(account_pubkey) => account_pubkey,
            _ => return Err(Error::InvalidAccountPubkey),
        };
        let mint_pubkey =
            Pubkey::from_str(&"4PYnraBJbdPXeMXdgL5k1m3TCcfNMaEWycvEQu2cteEV").unwrap();
        let receiver_pubkey =
            Pubkey::from_str(&"6GBLiSwAPhDMttmdjo3wvEsssEnCiW3yZwVyVZnhFm3G").unwrap();
        let source_pubkey = spl_associated_token_account::get_associated_token_address(
            &account_pubkey,
            &mint_pubkey,
        );
        let destination_pubkey = spl_associated_token_account::get_associated_token_address(
            &receiver_pubkey,
            &mint_pubkey,
        );
        let transfer_instruction = match spl_token::instruction::transfer(
            &spl_token::ID,
            &source_pubkey,
            &destination_pubkey,
            &account_pubkey,
            &[&account_pubkey],
            ctx.query.amount,
        ) {
            Ok(transfer_instruction) => transfer_instruction,
            _ => return Err(Error::InvalidInstruction),
        };
        let transaction_message = Message::new(&[transfer_instruction], None);

        Ok(Transaction::new_unsigned(transaction_message))
    }
}