function Inputfield() {
    return(
        <div className="flex flex-col justify-center items-center text-white pt-10">

            <p className=" text-2xl">Text Field</p>
            <input  type="text"
                    name="key_input"
                    id="key_input"
                    className="bg-white appearance-none border border-gray-400 rounded-full w-[15%] p-4 text-black leading-tight focus:outline-none focus:border-indigo-500 focus:border-[2px] text-center mt-3"
                    placeholder="Enter Code"
                    required></input>

        </div>
    );
}
export default Inputfield