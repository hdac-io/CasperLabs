// The entry file of your WebAssembly module.
import * as CL from "../../../../contract-as/assembly";
import {Error, ErrorCode} from "../../../../contract-as/assembly/error";
import {removeAssociatedKey, RemoveKeyFailure} from "../../../../contract-as/assembly/account";
import {typedToArray} from "../../../../contract-as/assembly/utils";
import {PublicKey} from "../../../../contract-as/assembly/key";

export function call(): void {
  let accountBytes = CL.getArg(0);
  if (accountBytes === null) {
    Error.fromErrorCode(ErrorCode.MissingArgument).revert();
    return;
  }

  const accountResult = PublicKey.fromBytes(accountBytes);
  if (accountResult.hasError()) {
    Error.fromErrorCode(ErrorCode.InvalidArgument).revert();
    return;
  }
  const account = accountResult.value;
  
  if (removeAssociatedKey(account) != RemoveKeyFailure.Ok) {
    Error.fromUserError(<u16>4464).revert();
    return;
  }
}
