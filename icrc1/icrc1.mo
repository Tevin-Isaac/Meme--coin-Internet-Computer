// Define the ICRC1 meme token canister
actor class ICRC1MemeToken {
  // Define the token identifier
  public let id : Nat;

  // Define the token name
  public var name : Text;

  // Define the token symbol
  public var symbol : Text;

  // Define the total supply of the token
  public var totalSupply : Nat;

  // Define the ledger to track token balances
  var ledger : HashMap<Principal, Nat>;

  // Define the meme URL associated with the token
  public var memeUrl : Text;

  // Initialize the ICRC1 meme token with the specified parameters
  public shared(msg) func init(id: Nat, name: Text, symbol: Text, totalSupply: Nat, memeUrl: Text) : async () {
    self.id := id;
    self.name := name;
    self.symbol := symbol;
    self.totalSupply := totalSupply;
    self.ledger := HashMap<Principal, Nat>();
    self.memeUrl := memeUrl;
  }

  // Transfer tokens from one account to another
  public shared(msg) func transfer(to: Principal, amount: Nat) : async Bool {
    if (self.ledger[msg.caller] >= amount) {
      self.ledger[msg.caller] -= amount;
      self.ledger[to] += amount;
      return true;
    } else {
      return false;
    }
  }

  // Get the balance of tokens for a specific account
  public query func balanceOf(account: Principal) : async Nat {
    switch (self.ledger.get(account)) {
      case (null) { 0 };
      case (some(balance)) { balance };
    }
  }
}