import { useEffect, useRef } from "react";

const ErrorToast = ({ duration = "default" ,message, button, onClose, isError }) =>{
    const onCloseRef = useRef(onClose);
    onCloseRef.current = onClose;
    useEffect(() => {
        if (duration === "sticky") {
          return;
        }
    
        const timer = setTimeout(() => {
          onCloseRef.current?.();
        }, 7000);
    
        return () => {
          clearTimeout(timer);
        };
      }, [duration]);

    return(
        <div className={`flex rounded-8 p-3 relative w-full items-center justify-center text-button transition-transform duration-300 ${isError ? "bg-[#FF0000]" : "bg-[#008000]"}`}
            data-testid="error-message"
            >
            {onClose ? (
                <div
                className={`flex absolute cursor-pointer`}
                style={{
                    top: 5,
                    right: 7,
                    width: 13,
                    height: 13,
                }}
                onClick={onClose}
                data-testid="close-btn"
                >
                <SolidPlus style={{ transform: "rotate(45deg)" }} />
                </div>
            ) : null}
      <div className={`flex space-x-4 items-center`}>
        <div className={`bold`}>{message}</div>
        {button}
      </div>
    </div>
    )
};

const SolidPlus = () => {

    return (
        <svg
          width={16}
          height={16}
          viewBox="0 0 16 16"
          fill="currentColor"
          xmlns="http://www.w3.org/2000/svg"
        >
          <g clipPath="url(#clip12)">
            <path
              fillRule="evenodd"
              clipRule="evenodd"
              d="M8 0C8.55228 0 9 0.447715 9 1V15C9 15.5523 8.55228 16 8 16C7.44772 16 7 15.5523 7 15V1C7 0.447715 7.44772 0 8 0Z"
              fill="#DEE3EA"
            />
            <path
              fillRule="evenodd"
              clipRule="evenodd"
              d="M0.000976562 8C0.000976562 7.44772 0.448692 7 1.00098 7H15.001C15.5533 7 16.001 7.44772 16.001 8C16.001 8.55228 15.5533 9 15.001 9H1.00098C0.448692 9 0.000976562 8.55228 0.000976562 8Z"
              fill="#DEE3EA"
            />
          </g>
          <defs>
            <clipPath id="clip12">
              <rect width="16" height="16" fill="white" />
            </clipPath>
          </defs>
        </svg>
      );
};

export default ErrorToast;