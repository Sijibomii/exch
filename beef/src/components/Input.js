import { forwardRef } from "react";

const Input = forwardRef(
  ({ className, textarea, error, transparent, ...props }, ref) => {
    const bg = transparent ? `bg-transparent` : `bg-[#242c37]`;
    const ring = error ? `ring-1 ring-[#5575e7]` : "";
    const cn = `w-full py-2 px-4 rounded-8 text-[#dee3ea] placeholder-[#5d7290] focus:outline-none ${bg} ${ring} ${className} `;

    return textarea ? (
      <textarea
        ref={ref}
        className={cn}
        data-testid="textarea"
        {...(props)}
      />
    ) : (
      <input ref={ref} className={cn} data-testid="input" {...props} />
    );
  }
);

Input.displayName = "Input";

export default Input;