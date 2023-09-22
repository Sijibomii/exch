
import Chart from "../components/Chart";
import Input from "../components/Input";
import { Formik, Form } from "formik";
import InputErrorMsg from "../components/InputErrorMsg";
import { WaitForWsAndAuth } from "../lib/WaitForWs";

const TradingPage = () => {
    return(
        <WaitForWsAndAuth>
            <Trade />
        </WaitForWsAndAuth>
    );
}

const Trade = () => {

    
    
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
                                  amount: 0
                                }
                              }
                              validateOnChange
                              validateOnBlur 
              
                              validate={({ qty, amount }) => {
                                return {};
                            }}
                            onSubmit={async ({ qty, amount }) => {
                            
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
                                        amount: 0
                                        }
                                    }
                                    validateOnChange
                                    validateOnBlur 
                    
                                    validate={({ qty, amount }) => {
                                        return {};
                                    }}
                                    onSubmit={async ({ qty, amount }) => {
                                    
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