@include('layouts.body-header')

<section class="h-screen">
    <div class="h-full">
    <!-- Left column container with background-->
        <div class="flex h-full items-center justify-center">
        <!-- Right column container -->
            <div class="mb-12 md:mb-0 md:w-8/12 lg:w-5/12 xl:w-5/12">
                <span class="text-xl font-bold text-slate-700">Signin to your account.</span>
                <form class="pt-3">
                    <x-bladewind::input type="email" name="email" required="true" label="Email Address"  />
                    <x-bladewind::input type="password" name="password" required="true" viewable="true" label="Password" suffix="eye"  />

                    <div class="mb-6 grid grid-cols-2 justify-center text-slate-600 items-center">
                        <x-bladewind::checkbox label="Remember me" class="m-0 p-0"  />
                        <a href="#!" class="text-sm text-right sm:text-md hover:text-blue-500 hover:underline duration-300 transition-all ease-in">Forgot password?</a>
                    </div>

                    <div class="text-center lg:text-left">
                        <x-bladewind::button type="blue" show_focus_ring="false" size="small" can_submit="true" class="w-full text-white font-bold">Login</x-bladewind::button>

                        <p class="mb-0 mt-2 pt-1 text-sm text-slate-600 font-semibold">
                            Don't have an account?
                            <a href="{{ route('register') }}" class="transition duration-150 ease-in-out hover:text-blue-600 focus:text-blue-600 active:text-blue-700" >Register Now!</a >
                        </p>
                    </div>
                </form>
            </div>
        </div>
    </div>
</section>

@include('layouts.body-footer')
