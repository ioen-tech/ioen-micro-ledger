const routes = [
  {
    path: '/supply-agreements',
    component: () => import('@/layouts/basic/Index.vue'),
    children: [
      {
        path: '',
        name: 'Supply Agreements',
        component: () => import('@/views/supply-agreements/Index.vue')
      }
    ]
  }
]

export default routes
