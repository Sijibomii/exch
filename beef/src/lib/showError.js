import { useErrorToastStore } from "./useErrorStore";

const ShowError = (m) => {
    console.log("showErrorToast: ", m);
    useErrorToastStore.getState().showToast({ message: m });
};

export default ShowError;