pragma solidity ^0.4.25;

contract Circus {
  function get_party_id(uint16 mp_id) public returns (uint16 request_id);
}

  contract CircusTest {
    address circus;

    event TestLog(string msg);

    constructor(address c) public {
      circus = c;
    }

    function run(uint16 mp_id) public {
      emit TestLog("Executing get_party_id");
      Circus(circus).get_party_id(mp_id);
      emit TestLog("Executed get_party_id");
  }

  function __callback(uint16, string) public {
    emit TestLog("received response");
  }
}
