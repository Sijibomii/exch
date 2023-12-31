import { useErrorToastStore, useInfoToastStore } from "../lib/useErrorStore"; 
import ErrorToast from "./ErrorToast"; 

const Error = () => {
    const { toasts, hideToast } = useErrorToastStore();
    
    return (
        <div
          style={{ zIndex: 1001 }}
          className={`flex w-full fixed top-0 justify-center`}
        >
            <div />
            <div className={`flex flex-col w-full`}>
              {toasts.map((t) => (
                <div key={t.id} className={`flex mb-3`}>
                  <ErrorToast
                    isError={true}
                    message={t.message}
                    duration={t.duration}
                    onClose={() => hideToast(t.id)}
                  />
                </div>
              ))}
            </div>
            <div />
        </div>
      );
};

export const Info = () => {
  const { toasts, hideToast } = useInfoToastStore();
  
  return (
      <div
        style={{ zIndex: 1001 }}
        className={`flex w-full fixed top-0 justify-center`}
      >
          <div />
          <div className={`flex flex-col w-full`}>
            {toasts.map((t) => (
              <div key={t.id} className={`flex mb-3`}>
                <ErrorToast
                isError={false}
                  message={t.message}
                  duration={t.duration}
                  onClose={() => hideToast(t.id)}
                />
              </div>
            ))}
          </div>
          <div />
      </div>
    );
};

export default Error;