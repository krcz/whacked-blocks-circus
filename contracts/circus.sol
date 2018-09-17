pragma solidity ^0.4.25;

contract Callbackable {
  function __callback(uint16 request_id, string value) public;
}


contract Circus {
  event GetPartyRequest(address _from, uint16 mp_id, uint16 request_id);

  uint16 cid;

  constructor() public {
    cid = 0;
  }

  function get_party_id(uint16 mp_id) public returns (uint16 request_id) {
    cid = cid + 1;
    emit GetPartyRequest(msg.sender, mp_id, cid);
    return cid;
  }

  function reply(address to, string value, uint16 request_id) public {
    Callbackable(to).__callback(request_id, value);
  }
}
