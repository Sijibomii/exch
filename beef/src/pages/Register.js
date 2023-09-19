import { useCallback } from "react";
import { Form, Formik } from "formik";
import InputErrorMsg from "../components/InputErrorMsg";
import Input from "../components/Input";



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

                  // const errors = {};

                  // const emailNotValid =  (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email))
                  // if (emailNotValid && email.length !== 0){ 
                  //   errors.email = "enter a valid email" 
                  //   return errors
                  // }
                  // setIsValid(password)
                  // if(!isValid && password.length !== 0){
                  //   errors.password= passwordErrors
                  //   return errors
                  // }

                  // had to do this because formik keeps changing my type of errors 
                  return {};
                }}
                
                onSubmit={async ({ email, password, captcha_code }) => {
                  
                  if (email.length === 0 || password.length ===0) return

                  if (captcha_code.length === 0){
                    alert('generate captcha with the link in the form')
                  }
                    
                    // const resp = await wrappedClient.login(email, password, captcha_code)
                    // if(resp.code===200 && resp.message==="SUCCESS"){
                    //   localStorage.setItem("@task/token", resp.data?.accessToken);
                    //   localStorage.setItem("@task/refresh-token", resp.data?.refreshToken);
                    //   push("/projects");
                    // }else{
                    //   alert(resp.message)
                    // }
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
