function Inputfield() {

    return(
        <div className="flex flex-col text-white">
            <p className=" text-2xl">Test</p>

            <input type="text" name="key_input" id="key_input" className="bg-white appearance-none border border-gray-400 rounded-full w-[20%] py-4 px-4 text-black leading-tight focus:outline-none focus:border-indigo-500 focus:border-[2px] text-center" placeholder="Enter Code" required></input>

        </div>
    );
}
export default Inputfield