@include('layouts.body-header')

<section class="h-screen">
    <div class="h-full">
    <!-- Left column container with background-->
        <div class="flex h-full items-center justify-center">
        <!-- Right column container -->
            <div class="mb-12 md:mb-0 md:w-8/12 lg:w-5/12 xl:w-5/12">
                <span class="text-xl font-bold text-slate-700">Signup to your account.</span>
                <form class="pt-3">
                    <div class="flex sm:grid sm:grid-cols-2 gap-3 items-center">
                        <x-bladewind::input type="number" name="id_number" required="true" label="ID number"  />
                        <x-bladewind::input name="department" required="true" label="Department"  />
                    </div>
                    <div class="flex sm:grid sm:grid-cols-2 gap-3 items-center">
                        <x-bladewind::input name="first_name" required="true" label="First name"  />
                        <x-bladewind::input name="last_name" required="true" label="Last name"  />
                    </div>
                    <x-bladewind::input type="email" name="email_add" required="true" label="Email address"  />
                    <x-bladewind::input type="password" name="password" required="true" viewable="true" label="Password" suffix="eye"  />

                    <div class="text-center lg:text-left">
                        <x-bladewind::button type="blue" show_focus_ring="false" size="small" can_submit="true" class="w-full text-white font-bold">Register</x-bladewind::button>

                        <p class="mb-0 mt-2 pt-1 text-sm text-slate-600 font-semibold">
                            Already have an account?
                            <a href="{{ route('login') }}" class="transition duration-150 ease-in-out hover:text-blue-600 focus:text-blue-600 active:text-blue-700" >Signin Now!</a >
                        </p>
                    </div>
                </form>
            </div>
        </div>
    </div>
</section>

@include('layouts.body-footer')
