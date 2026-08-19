import { FillEvent } from "../types/events";

export interface ParsedOrderFill {
  maker: string;
  taker: string;
  side: string;
  price: number;
  quantity: number;
  timestamp: number;
  signature: string;
  marketPubkey: string;
}

export default function parseOrderFillEvent(event: {
  name: string;
  data: FillEvent;
  signature: string;
}): ParsedOrderFill | null {
  if (!event?.data) return null;

  const { maker, taker, side, price, baseLotsFilled, timestamp, marketPubkey } = event.data;

  const sideStr = side === "ask" ? "sell" : "buy";

  return {
    maker: maker.toString(),
    taker: taker.toString(),
    side: sideStr,
    price: price.toNumber(),
    quantity: baseLotsFilled.toNumber(),
    timestamp: timestamp.toNumber(),
    signature: event.signature,
    marketPubkey: marketPubkey.toString(),
  };
}
