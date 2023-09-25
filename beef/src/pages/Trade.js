import Chart from "../components/Chart";
import Input from "../components/Input";
import { Formik, Form } from "formik";
import InputErrorMsg from "../components/InputErrorMsg";
import { WaitForWsAndAuth } from "../lib/WaitForWs";
import { useWrappedConn } from "../lib/useConnection";
import { useLocation } from "react-router-dom"
import { useEffect } from "react";

const TradingPage = () => {
    return(
        <WaitForWsAndAuth>
            <Trade />
        </WaitForWsAndAuth>
    );
}

const Trade = () => {
    const conn = useWrappedConn();
    const location = useLocation();
    const handler = (data, ref) => {
      console.log("GOT DATA: ", data);
      console.log("GOT REF: ", ref) 
    };

    async function sub(){
      console.log("subsc")
      await conn.subscribe.newTradeMsg(handler);
    }

    useEffect(() => {
       sub();
    }, [])
    function extractIdFromPath(path) {
        // Define a regular expression to match the "/trade/" followed by digits
        const regex = /\/trade\/(\d+)/;
        
        // Use the exec method to search for a match in the path
        const match = regex.exec(path);
        
        // Check if there is a match and return the captured ID or null
        if (match && match[1]) {
          return match[1]; // The ID is captured in the first capturing group (match[1])
        } else {
          return null; // Return null if no match is found
        }
      }
    return (
        <div className="">
            <Chart />
            <div className="">
                <div className="max-w-7xl m-auto flex items-center py-16">
                    <div className="buy w-1/2 mr-8">
                        <Formik
                            initialValues={
                                {
                                  qty: 0,
                                  price: 0
                                }
                              }
                              validateOnChange
                              validateOnBlur 
              
                              validate={({ qty, price }) => {
                                return {};
                            }}
                            onSubmit={async ({ qty, price }) => {
                              console.log(qty, price)
                                conn.mutation.sendTrade(
                                    extractIdFromPath(location.pathname),
                                    "BUY",
                                    qty,
                                    price
                                  )
                            }}
                            >
                        {({ isSubmitting, errors, handleChange, handleBlur, setFieldValue }) => (
                  <Form className={``}>
                    <div className="flex flex-col gap-4">
                    <div className="flex flex-col">
                        <h3 className="text-[#dee3ea] text-sm text-gray mb-2">Quantity:</h3>
                        {errors.qty ? (
                          <div className={`flex mt-1`}>
                            <InputErrorMsg>{errors.qty}</InputErrorMsg>
                          </div>
                        ) : null }
                          <Input
                            autoFocus
                            className={`qty-input`}
                            placeholder={"Enter Quantity of token"}
                            name="qty"
                            id="qty"
                            type={"qty"}
                            onBlur={handleBlur}
                            onChange={handleChange}
                          />
                    </div>
                    <div className="flex flex-col">
                        <h3 className="text-[#dee3ea] text-sm mb-2">Price</h3>
                        {errors.price ? (
                          <div className={`flex flex-col mt-1`}>
                            {errors.price.map(error => 
                            <InputErrorMsg
                            key={error.message}>{error.message}</InputErrorMsg>)}
                          </div>
                        ) : null}
                        <Input
                          className={`price-input`}
                          id="price"
                          placeholder={"Enter price"}
                          name="price"
                          type={"price"}
                          onBlur={handleBlur}
                          onChange={handleChange}
                          />
                    </div>
                    
                    <button loading={isSubmitting} type="submit" className="bg-[green] py-2">
                        BUY
                    </button>
                </div>
                  </Form>
                )}

                </Formik>
                    </div>
                    <div className="ml-8 sell w-1/2">
                            <Formik
                                    initialValues={
                                        {
                                        qty: 0,
                                        price: 0
                                        }
                                    }
                                    validateOnChange
                                    validateOnBlur 
                    
                                    validate={({ qty, price }) => {
                                        return {};
                                    }}
                                    onSubmit={async ({ qty, price }) => {
                                        conn.mutation.sendTrade(
                                            extractIdFromPath(location.pathname),
                                            "SELL",
                                            qty,
                                            price
                                          )
                                    }}
                                    >
                                {({ isSubmitting, errors, handleChange, handleBlur, setFieldValue }) => (
                        <Form className={``}>
                            <div className="flex flex-col gap-4">
                            <div className="flex flex-col">
                                <h3 className="text-[#dee3ea] text-sm text-gray mb-2">Quantity:</h3>
                                {errors.qty ? (
                                <div className={`flex mt-1`}>
                                    <InputErrorMsg>{errors.qty}</InputErrorMsg>
                                </div>
                                ) : null }
                                <Input
                                    autoFocus
                                    className={`qty-input`}
                                    placeholder={"Enter Quantity of token"}
                                    name="qty"
                                    id="qty"
                                    type={"qty"}
                                    onBlur={handleBlur}
                                    onChange={handleChange}
                                />
                            </div>
                            <div className="flex flex-col">
                                <h3 className="text-[#dee3ea] text-sm mb-2">Price</h3>
                                {errors.price ? (
                                <div className={`flex flex-col mt-1`}>
                                    {errors.price.map(error => 
                                    <InputErrorMsg
                                    key={error.message}>{error.message}</InputErrorMsg>)}
                                </div>
                                ) : null}
                                <Input
                                className={`price-input`}
                                id="price"
                                placeholder={"Enter price"}
                                name="price"
                                type={"price"}
                                onBlur={handleBlur}
                                onChange={handleChange}
                                />
                            </div>
                            
                            <button loading={isSubmitting} type="submit" className="bg-[red] py-2">
                                SELL
                            </button>
                        </div>
                        </Form>
                        )}

                        </Formik>
                    </div>
                </div>
            </div>
        </div>
    );
};


export default TradingPage;