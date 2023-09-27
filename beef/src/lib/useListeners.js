import { create } from "zustand";
import { combine } from "zustand/middleware";


export const useListenersStore = create(
  combine(
    {
      listeners: [],
    },
    (set) => ({
        setListeners: (listeners) => {
            set({
              listeners
          });
        },
        appendListener: (listeners) => {
            set((state) => ({
                listeners: [...state.listeners, listeners],
            })); 
        }
    })
  )
);