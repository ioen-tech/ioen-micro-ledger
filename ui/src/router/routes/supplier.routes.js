const routes = [
  {
    path: '/supplier',
    component: () => import('@/layouts/basic/Index.vue'),
    children: [
      {
        path: '',
        name: 'Producer',
        component: () => import('@/views/supplier/Index.vue')
      }
    ]
  }
]

export default routes
