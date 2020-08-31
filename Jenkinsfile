// this guarantees the node will use this template
def imageName = "herald"
def registry = "https://dreg.vsix.me:9443"
def credential = "dreg-registry"
def label = "herald-build"
def tag = null

def abortBuildIfTriggeredBySkippableCommit() {
    def changeSetCount = 0;
    def ciSkipCount = 0;
    if (currentBuild.changeSets != null) {
        for (changeSetList in currentBuild.changeSets) {
            for (changeSet in changeSetList) {
                changeSetCount++;
                if (changeSet.msg.contains('[ci-skip]')) {
                    ciSkipCount++;
                }
            }
        }
    }

    if (changeSetCount > 0 && changeSetCount == ciSkipCount) {
        currentBuild.result = 'NOT_BUILT'
        error("Stopping to prevent auto trigger. All commits contained [ci-skip]")
    }
}



podTemplate(imagePullSecrets: [credential],label: label,idleMinutes: 30,
  containers: [
    containerTemplate(name: 'jnlp', image: 'dreg.vsix.me:9443/jnlp-docker:c33362f', args: '${computer.jnlpmac} ${computer.name}'),
    containerTemplate(name: 'rust-nightly', image: 'dreg.vsix.me:9443/rust_nightly:20200828-05', command: 'cat', ttyEnabled: true),
    ],
    volumes: [
    hostPathVolume(hostPath: '/var/run/docker.sock', mountPath: '/var/run/docker.sock'),
    persistentVolumeClaim(mountPath: '/workspace', claimName: 'herald-build-cache', readOnly: false)
    ]
      ) {
    node(label) {
                dir("/workspace/herald") {
            container("rust-nightly") {
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
                    stage('Release-Build') {
                        sh '''
                        cargo build --release
                        '''
                    }
            }
            container("jnlp") {
                    stage('Package') {
               docker.withRegistry(registry, credential) {
                        app = docker.build(imageName)
               }
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
            stage('Update Chart') {
                container('dokubectl') {
                    datever = sh(returnStdout: true, script: "date +%Y%m%d").trim()
                    sh "/app/yq w -i ./helm/herald/Chart.yaml version ${datever}-${hash}"
                    sh "/app/yq w -i ./helm/herald/Chart.yaml appVersion ${datever}-${hash}"
                    sh 'helm push -f ./helm/herald vsix'
                }
            }
            stage('Deploy') {
                build(
                    job: 'support/update-flux',
                    parameters: [
                        string(name: "K8S_NAMESPACE", value: "monitoring"),
                        string(name: "K8S_HELMRELEASE", value: "herald"),
                        string(name: "HELM_CHARTVER", value: "${datever}-${hash}"),
                        string(name: "IMAGE_TAG", value: "${hash}"),
                        string(name: "KUSTOMIZE_PATH", value: "deploy/keim")
                        ])
                    currentBuild.result = 'SUCCESS'
            }
        }
                }
            }
        }
