import { createRouter, createWebHistory } from 'vue-router'
import HomeViews from "../src/views/homeViews.vue";
import ToolsViews from "../src/views/toolsViews.vue";
import SchematicViews from "../src/views/schematicsViews.vue"
import IndividuationViews from "../src/views/individuationViews.vue"
import ReportViews from "../src/views/reportViews.vue";
import AboutViews from "../src/views/aboutViews.vue";
const routes = [
    {
        path: '/',
        redirect: '/home'
    }, {
        path: '/home',
        name: 'home',
        meta: {
            title: 'Home',
            description: '',
            keywords: ''
        },
        component: HomeViews
    },{
        path: '/tools',
        name: 'tools',
        meta: {
            title: 'Tools',
            description: '',
            keywords: ''
        },
        component: ToolsViews
    },{
        path: '/schematic',
        name: 'schematic',
        meta: {
            title: 'Schematic',
            description: '',
            keywords: ''
        },
        component: SchematicViews
    },{
        path: '/individuation',
        name: 'individuation',
        meta: {
            title: 'individuation',
            description: '',
            keywords: ''
        },
        component: IndividuationViews
    },{
        path: '/report',
        name: 'report',
        meta: {
            title: 'report',
            description: '',
            keywords: ''
        },
        component: ReportViews
    },{
        path: '/about',
        name: 'about',
        meta: {
            title: 'about',
            description: '',
            keywords: ''
        },
        component: AboutViews
    }
]

const router = createRouter({
    history: createWebHistory(),
    routes
})

router.beforeEach((to, from, next) => {
    document.title = to.meta.title || 'Default Title'
    const description = to.meta.description || 'Default Description'
    const keywords = to.meta.keywords || 'default, keywords'

    let metaDescription = document.querySelector('meta[name="description"]')
    if (metaDescription) {
        metaDescription.setAttribute('content', description)
    } else {
        metaDescription = document.createElement('meta')
        metaDescription.name = 'description'
        metaDescription.content = description
        document.head.appendChild(metaDescription)
    }

    let metaKeywords = document.querySelector('meta[name="keywords"]')
    if (metaKeywords) {
        metaKeywords.setAttribute('content', keywords)
    } else {
        metaKeywords = document.createElement('meta')
        metaKeywords.name = 'keywords'
        metaKeywords.content = keywords
        document.head.appendChild(metaKeywords)
    }

    next()
})

export default router
