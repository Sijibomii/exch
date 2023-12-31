import { create } from "zustand";
import { combine } from "zustand/middleware";


export const useOrderBookStore = create(
  combine(
    {
      orders: [],
      chart: null
    },
    (set) => ({
        setOrderBook: (orders) => {
            set({
              orders
          });
        },
        appendOrder: (order) => {
            set((state) => ({
                orders: [...state.orders, order],
            })); 
        }
    })
  )
);