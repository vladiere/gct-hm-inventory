<?php

use Illuminate\Support\Facades\Route;

Route::prefix("/")->group(function () {
    Route::get("", function () {
        notify()->error("Welcome to Laravel Notify ⚡️");
        return view("pages.index_page");
    })->name("index");

    Route::get("inventory", function () {
        return view("pages.inventory_page");
    })->name("inventory");

    Route::get("circulation", function () {
        return view("pages.circulation_page");
    })->name("circulation");

    Route::get("penalty", function () {
        return view("pages.penalty_page");
    })->name("penalty");

    Route::get("table", function () {
        return view("pages.table_page");
    })->name("table");
});

Route::prefix("auth")->group(function () {
    Route::get("login", function () {
        return view("pages.login_page");
    })->name("login");
    Route::get("register", function () {
        return view("pages.register_page");
    })->name("register");
});
