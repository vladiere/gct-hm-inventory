@include('layouts.body-header')

<div class="h-screen w-screen flex flex-col">
    @include('layouts.header')
    <div class="h-full w-full grow overflow-y-auto">
        @yield('content')
        @include('layouts.footer')
    </div>
</div>

@include('layouts.body-footer')
