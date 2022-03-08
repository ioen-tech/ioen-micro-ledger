const routes = [
  {
    path: '/supplier',
    component: () => import('@/layouts/basic/Index.vue'),
    children: [
      {
        path: '',
        name: 'Supplier',
        component: () => import('@/views/supplier/Index.vue')
      }
    ]
  }
]

export default routes
