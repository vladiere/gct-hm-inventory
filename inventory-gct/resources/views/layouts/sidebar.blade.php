@php
$links = [
    ['icon' => 'home', 'title' => 'Dashboard', 'to' => 'index'],
    ['icon' => 'square-3-stack-3d', 'title' => 'Inventory', 'to' => 'inventory'],
    ['icon' => 'home', 'title' => 'Circulations', 'to' => 'circulation'],
    ['icon' => 'exclamation-triangle', 'title' => 'Penalties', 'to' => 'penalty'],
    ['icon' => 'table-cells', 'title' => 'Tables', 'to' => 'table'],
]
@endphp


<div id="sidebar" class="hidden absolute z-50 h-screen w-screen">
    <div class="flex h-full w-full duration-300 transition-all ease-in-out">
        <div class="w-1/3 flex flex-col col-span-1 h-full bg-slate-50 p-3">
            <div class="flex items-center justify-between">
                <h1 class="text-2xl font-bold">Dashboard</h1>
                <div onclick="showSidebar()">
                    <x-bladewind::icon name="x-mark" class="cursor-pointer" />
                </div>
            </div>
            <hr class="bg-slate-400 my-2" />
            <x-bladewind::list-view transparent="true" class="w-full">
                @foreach ($links as $link)
                    <x-bladewind::list-item class="flex items-center py-2 px-3 hover:bg-slate-300 duration-300 transition-all ease-linear w-full hover:rounded-md">
                        <a href="{{ route($link['to']) }}" class="flex items-center w-full">
                            <x-bladewind::icon name="{{ $link['icon'] }}" />
                            <div class="ml-3">
                                <div class="text-sm font-medium text-slate-900">{{ $link['title'] }}</div>
                            </div>
                        </a>
                    </x-bladewind::list-item>
                @endforeach
            </x-bladewind::list-view>
        </div>
        <div onclick="showSidebar()" class="bg-stone-900 opacity-60 h-full w-full"></div>
    </div>
</div>
