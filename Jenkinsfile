// this guarantees the node will use this template
def imageName = "herald"
def registry = "https://dreg.vsix.me:9443"
def credential = "dreg-registry"
def label = "herald-build"
def tag = null
podTemplate(imagePullSecrets: [credential],label: label,idleMinutes: 30,
  containers: [
    containerTemplate(name: 'jnlp', image: 'dreg.vsix.me:9443/jnlp-docker:latest', args: '${computer.jnlpmac} ${computer.name}'),
    containerTemplate(name: 'rust-nightly', image: 'dreg.vsix.me:9443/rust_nightly:20200828-05', command: 'cat', ttyEnabled: true),
    ],
    volumes: [
    hostPathVolume(hostPath: '/var/run/docker.sock', mountPath: '/var/run/docker.sock'),
    persistentVolumeClaim(mountPath: '/workspace', claimName: 'herald-build-cache', readOnly: false)
    ]
      ) {
    node(label) {
            container("rust-nightly") {
                dir("/workspace/herald") {
                    stage('Checkout') {
                        checkout scm
                        tag = sh(returnStdout: true, script: "git describe --tags || echo 'none'").trim()
                        hash = sh(returnStdout: true, script: "git rev-parse --short HEAD").trim()
                    }
                    stage('Test') {
                        sh '''
                        ./checktest.sh
                        '''
                        cobertura coberturaReportFile: 'cobertura.xml'
                    }
                    stage('Build') {
                        intermediate = docker.build("herald-latest","-f Dockerfile.intermediate")
                        app = docker.build(imageName)
                    }
                    stage('Push') {
                       docker.withRegistry(registry, credential) {
                            app.push("latest")
                            if (tag == 'none') {
                                app.push(hash)
                            } else {
                                app.push(tag)
                                app.push(hash)
                            }
                       }
                    }
        }
                }
            }
        }
