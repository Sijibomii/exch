import { useInfoToastStore } from "./useErrorStore";

const ShowInfo = (m) => { 
    console.log("showInfoToast: ", m);
    useInfoToastStore.getState().showToast({ message: m });
};

export default ShowInfo; 