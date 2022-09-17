use simple_blockchain_rs::tests::*;

fn main() {
    client_class_print_key();
    transaction_class_print_transaction();
    transaction_class_print_multiple_transactions();
    block_class_print_block();
    blockchain_class_print_blockchain();
    block_mine();
    push_block_into_blockchain();
}
