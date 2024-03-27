// Define the MemeToken canister
actor MemeToken {

  // Define the token balance mapping
  var balances : HashMap<Principal, Nat> = HashMap();

  // Define the token metadata
  var name : Text = "MemeToken";
  var symbol : Text = "MEME";
  var totalSupply : Nat = 0;

  // Function to mint new tokens
  public shared({caller}) func mint(amount : Nat) : async Nat {
    assert(caller == Principal.fromActor(this));
    let newTotalSupply = totalSupply + amount;
    totalSupply := newTotalSupply;
    let balance = balances.getOrDefault(caller, 0);
    let newBalance = balance + amount;
    balances.put(caller, newBalance);
    newBalance;
  };

  // Function to transfer tokens to another account
  public shared({caller}) func transfer(to : Principal, amount : Nat) : async Bool {
    assert(amount > 0);
    let fromBalance = balances.getOrDefault(caller, 0);
    assert(fromBalance >= amount);
    let toBalance = balances.getOrDefault(to, 0);
    balances.put(caller, fromBalance - amount);
    balances.put(to, toBalance + amount);
    true;
  };

  // Function to burn tokens
  public shared({caller}) func burn(amount : Nat) : async Bool {
    assert(amount > 0);
    let fromBalance = balances.getOrDefault(caller, 0);
    assert(fromBalance >= amount);
    let newTotalSupply = totalSupply - amount;
    totalSupply := newTotalSupply;
    balances.put(caller, fromBalance - amount);
    true;
  };

  // Function to pause transfers
  public shared({caller}) func pauseTransfers() : async Bool {
    assert(caller == Principal.fromActor(this));
    // Implement logic to pause transfers
    true;
  };

  // Function to unpause transfers
  public shared({caller}) func unpauseTransfers() : async Bool {
    assert(caller == Principal.fromActor(this));
    // Implement logic to unpause transfers
    true;
  };

  // Function to check the balance of a specific account
  public query func balanceOf(account : Principal) : async Nat {
    balances.getOrDefault(account, 0);
  };

  // Function to retrieve the token metadata
  public query func getTokenMetadata() : async {name : Text; symbol : Text; totalSupply : Nat} {
    {name = name; symbol = symbol; totalSupply = totalSupply};
  };
};