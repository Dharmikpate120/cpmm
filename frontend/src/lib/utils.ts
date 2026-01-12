import { type ClassValue, clsx } from 'clsx'
import {
  Address,
  getAddressEncoder,
  getProgramDerivedAddress,
  ProgramDerivedAddress,
  ProgramDerivedAddressBump,
} from 'gill'
import { twMerge } from 'tailwind-merge'

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function ellipsify(str = '', len = 4, delimiter = '..') {
  const strLen = str.length
  const limit = len * 2 + delimiter.length

  return strLen >= limit ? str.substring(0, len) + delimiter + str.substring(strLen - len, strLen) : str
}
const addressEncoder = getAddressEncoder()

export async function get_lexicographical_token_pda(
  a: Address,
  b: Address,
  program_id: Address,
): Promise<readonly [Address<string>, ProgramDerivedAddressBump]> {
  if (a.toString() > b.toString()) {
    return await getProgramDerivedAddress({
      programAddress: program_id,
      seeds: [addressEncoder.encode(b), addressEncoder.encode(a)],
    })
  } else {
    return await getProgramDerivedAddress({
      programAddress: program_id,
      seeds: [addressEncoder.encode(a), addressEncoder.encode(b)],
    })
  }
}

export async function getLexicographicalMintPda(
  str: string,
  ammTokenAccount: Address,
  a: Address,
  b: Address,
  program_id: Address,
): Promise<readonly [Address<string>, ProgramDerivedAddressBump]> {
  if (a.toString() > b.toString()) {
    return await getProgramDerivedAddress({
      programAddress: program_id,
      seeds: [str, addressEncoder.encode(ammTokenAccount), addressEncoder.encode(b), addressEncoder.encode(a)],
    })
  } else {
    return await getProgramDerivedAddress({
      programAddress: program_id,
      seeds: [str, addressEncoder.encode(ammTokenAccount), addressEncoder.encode(a), addressEncoder.encode(b)],
    })
  }
}

export async function getLexicographicalLpTokenPda(
  ownerAddress: Address,
  ammTokenAccount: Address,
  a: Address,
  b: Address,
  program_id: Address,
): Promise<readonly [Address<string>, ProgramDerivedAddressBump]> {
  if (a.toString() > b.toString()) {
    return await getProgramDerivedAddress({
      programAddress: program_id,
      seeds: [
        addressEncoder.encode(ownerAddress),
        addressEncoder.encode(ammTokenAccount),
        addressEncoder.encode(b),
        addressEncoder.encode(a),
      ],
    })
  } else {
    return await getProgramDerivedAddress({
      programAddress: program_id,
      seeds: [
        addressEncoder.encode(ownerAddress),
        addressEncoder.encode(ammTokenAccount),
        addressEncoder.encode(a),
        addressEncoder.encode(b),
      ],
    })
  }
}
