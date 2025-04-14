import { createRouter, createWebHistory } from 'vue-router'
import HomeViews from "../src/views/homeViews.vue";
import ToolsViews from "../src/views/toolsViews.vue";
import SchematicViews from "../src/views/schematicsViews.vue"
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
