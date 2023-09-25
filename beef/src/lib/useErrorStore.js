import { create } from "zustand";
import { combine } from "zustand/middleware";
import { v4 } from "uuid";



export const useErrorToastStore = create(
  combine(
    {
      toasts: [],
    },
    (set) => ({
      hideToast: (id) =>
        set((x) => ({ toasts: x.toasts.filter((y) => y.id !== id) })),
      showToast: (t) =>
        set((x) => {
          const currentRemovedToasts = x.toasts.filter((y) => y.message !== t.message);
          return {
            toasts: [...currentRemovedToasts, { ...t, id: v4() }]
          };
        }),
    })
  )
);