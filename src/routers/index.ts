import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import Home from "../views/Home.vue";

const routes: Array<Readonly<RouteRecordRaw>> = [
    {
        path: "/",
        name: "Home",
        component: Home
    },
    {
        path: "/compress",
        name: "Compress",
        component: () => import("../views/Compress.vue")
    },
    {
        path: "/decompress",
        name: "Decompress",
        component: () => import("../views/Decompress.vue")
    },
    {
        path: "/options",
        name: "Options",
        component: () => import("../views/Options.vue"),
        children: [
            {
                path: "/options/grneral",
                name: "OptionsGeneral",
                component: () => import("../views/options/General.vue")
            },
            {
                path: "/options/formats",
                name: "OptionsFormats",
                component: () => import("../views/options/Formats.vue")
            },
            {
                path: "/options/about",
                name: "OptionsAbout",
                component: () => import("../views/options/About.vue")
            }
        ]
    }
    
];

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
});

export default router;
