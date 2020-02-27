//@ts-nocheck
import * as CL from "../../../../contract-as/assembly";
import {Error} from "../../../../contract-as/assembly/error";
import {fromBytesString} from "../../../../contract-as/assembly/bytesrepr";
import {Key} from "../../../../contract-as/assembly/key";
import {putKey} from "../../../../contract-as/assembly";
import {PurseId} from "../../../../contract-as/assembly/purseid";

enum Args {
  PurseName = 0,
}

enum CustomError {
  MissingPurseNameArg = 1,
  InvalidPurseNameArg = 2,
  UnableToCreatePurse = 3
}

export function call(): void {
  // purse name arg
  const purseNameArg = CL.getArg(Args.PurseName);
  if (purseNameArg === null) {
    Error.fromUserError(<u16>CustomError.MissingPurseNameArg).revert();
    return;
  }
  const purseNameResult = fromBytesString(purseNameArg);
  if (purseNameResult.hasError()) {
    Error.fromUserError(<u16>CustomError.InvalidPurseNameArg).revert();
    return;
  }
  let purseName = purseNameResult.value;

  const maybePurse = PurseId.create();
  if (maybePurse === null){
    Error.fromUserError(<u16>CustomError.UnableToCreatePurse).revert();
    return;
  }

  const key = Key.fromURef(maybePurse.asURef());

  putKey(purseName, <Key>key);
}
