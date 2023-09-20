import { useCallback, useEffect, useState } from "react";
import { Form, Formik } from "formik";
import InputErrorMsg from "../components/InputErrorMsg";
import Input from "../components/Input";
import { useTokenStore } from "../lib/useTokenStore";
import { useHttpClient } from "../lib/useHttpClient";
import { wrap } from "../lib/http"

export const RegisterButton= ({
  children,
  loading,
}) => {
  const query = "";
 
  const clickHandler = useCallback(() => {
    if (typeof query.next === "string" && query.next) {
      return
    }

  }, [query]);

  return (
    <button
      className="justify-center text-base py-3 mt-2 bg-[#E3A014]"
      disabled={false || loading}
      onClick={clickHandler}
    >
      <div
        className={Array.isArray(children) ? "grid gap-4": ""}
        style={{
          gridTemplateColumns: "1fr auto 1fr",
        }}
      >
        {Array.isArray(children) ? [ ...children] : children}
        <div />
      </div>
    </button>
  );
};


const Register = () => {

  const [tokensChecked, setTokensChecked] = useState(false);
  const hasTokens = useTokenStore((s) => !!(s.accessToken));
  const httpClient = useHttpClient();
  const wrappedClient = wrap(httpClient.http);


  useEffect(() => {
    if (hasTokens) {
      window.location = '/';
    } else {
      setTokensChecked(true);
    }
  }, [hasTokens]);

  if (!tokensChecked) return null;

    return (
      <div className="h-[87vh]">
        <div className="h-full flex items-center justify-center">
        <div className="flex  m-auto flex-col p-14 pt-0 gap-5 sm:rounded-8 z-10 sm:w-400 w-1/3">
            <div className="flex gap-2 flex-col text-center">
              <span className="text-3xl text-[#dee3ea] font-bold">Welcome</span>
            </div>
            <Formik 
                  initialValues={
                  {
                    email: "",
                    password: "",
                    captcha_code: ""
                  }
                }
                validateOnChange
                validateOnBlur 

                validate={({ email, password }) => {
                  //@todo: error validation
                }}
                
                onSubmit={async ({ email, password }) => {                    
                  const resp = await wrappedClient.register(email, password)
                  
                  if(resp.status===200){
                    window.location = '/login';
                  }else{
                    alert(resp.message)
                  }
                }}
              >
                {({ isSubmitting, errors, handleChange, handleBlur, setFieldValue }) => (
                  <Form className={``}>
                    <div className="flex flex-col gap-4">
                    <div className="flex flex-col">
                        <h3 className="text-[#dee3ea] text-sm text-gray">Email:</h3>
                        {errors.email ? (
                          <div className={`flex mt-1`}>
                            <InputErrorMsg>{errors.email}</InputErrorMsg>
                          </div>
                        ) : null }
                          <Input
                            autoFocus
                            className={`login-input`}
                            placeholder={"Enter your Email"}
                            name="email"
                            id="email"
                            type={"email"}
                            onBlur={handleBlur}
                            onChange={handleChange}
                          />
                    </div>
                    <div className="flex flex-col">
                        <h3 className="text-[#dee3ea] text-sm">Password</h3>
                        {errors.password ? (
                          <div className={`flex flex-col mt-1`}>
                            {errors.password.map(error => 
                            <InputErrorMsg
                            key={error.message}>{error.message}</InputErrorMsg>)}
                          </div>
                        ) : null}
                        <Input
                          className={`login-input`}
                          id="password"
                          placeholder={"Enter password"}
                          name="password"
                          type={"password"}
                          onBlur={handleBlur}
                          onChange={handleChange}
                          />
                    </div>
                    
                    <RegisterButton loading={isSubmitting} type="submit">
                        Register
                    </RegisterButton>
                </div>
                  </Form>
                )}
            </Formik>
            
          </div>
        </div>
      </div>
    )
}

export default Register;
