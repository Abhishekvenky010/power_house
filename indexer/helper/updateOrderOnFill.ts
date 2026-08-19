import prisma from "../lib/prisma";
import { OrderStatus } from "../generated/prisma/enums";

export async function updateOrdersOnFill(
  makerOrderId: number,
  takerOrderId: number,
  filledLots: number
): Promise<void> {
  if (filledLots <= 0) return;

  const orders = await prisma.order.findMany({
    where: {
      OR: [
        { orderId: String(makerOrderId) },
        { orderId: String(takerOrderId) },
      ],
    },
  });

  for (const order of orders) {
    const newFilledLots = order.filledLots + filledLots;
    let newStatus = order.status;

    if (newFilledLots >= order.baseLots) {
      newStatus = OrderStatus.filled;
    } else if (newFilledLots > 0 && newStatus === OrderStatus.Open) {
      newStatus = OrderStatus.Partial;
    }

    await prisma.order.update({
      where: { id: order.id },
      data: {
        filledLots: newFilledLots,
        status: newStatus,
      },
    });
  }
}
