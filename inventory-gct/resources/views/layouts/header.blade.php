@include('layouts.sidebar')
<div class="grid grid-cols-2 items-center px-5 py-3 bg-slate-100 w-full">
    <!-- Smile, breathe, and go slowly. - Thich Nhat Hanh -->
    <div class="flex items-center justify-start gap-2 w-full">
        <div onclick="showSidebar()">
            <x-bladewind::icon name="bars-3" class="cursor-pointer" />
        </div>
        <x-bladewind::avatar
            image="https://picsum.photos/id/123/1200/300"
            size="medium"
            class="m-0"
        />
        <span class="text-2xl font-bold">BSHM Inventory and Borrowing</span>
    </div>
    <div class="flex gap-5 items-center justify-end w-full">
        <x-bladewind::dropmenu class="h-60 overflow-y-scroll">
            <x-slot name="trigger">
                <x-bladewind::bell />
            </x-slot>
            <x-bladewind::dropmenu-item>
                <x-bladewind::list-view transparent="true">
                    @for ($i = 0; $i < 10; $i++)
                        <x-bladewind::list-item>
                            <x-bladewind::avatar size="small" image="https://picsum.photos/1200/300" />
                            <div class="mx-1 pt-1">
                                <div class="text-sm">
                                    <span class="font-medium text-slate-900">
                                        Michael
                                    </span>
                                    assigned <a href="#">a task</a> to you
                                    <div class="text-xs">3 hours ago</div>
                                </div>
                            </div>
                        </x-bladewind::list-item>
                    @endfor

                </x-bladewind::list-view>
            </x-bladewind::dropmenu-item>
        </x-bladewind::dropmenu>

        <x-bladewind::dropmenu trigger="cog-6-tooth-icon">
            <x-bladewind::dropmenu-item>
                Settings
            </x-bladewind::dropmenu-item>
            <x-bladewind::dropmenu-item>
                Profile
            </x-bladewind::dropmenu-item>
            <x-bladewind::dropmenu-item>
                Logout
            </x-bladewind::dropmenu-item>
        </x-bladewind::dropmenu>
    </div>
</div>

<script>
  showSidebar = () => {
    let sidebar = document.querySelector("#sidebar");
    sidebar.classList.toggle("hidden");
  };
</script>
